mod temporaries;
mod transformers;

use std::collections::{HashMap, HashSet};

use errors::TransformerErrors;
use nilang_types::{
    instructions::Instruction,
    nodes::{Node, Program},
};
use temporaries::Temporaries;

/// (size, <field, offset>)
type StructSize = (usize, HashMap<Box<str>, usize>);

#[derive(Debug)]
struct TypesRef {
    alignment: usize,
    primitives: HashMap<Box<str>, usize>,
    structures: HashMap<Box<str>, StructSize>,
}

impl Default for TypesRef {
    fn default() -> Self {
        let alignment = 8;
        Self {
            alignment,
            primitives: HashMap::from([
                ("ptr".into(), alignment),
                ("int".into(), alignment),
                ("bool".into(), 1),
                ("char".into(), 1),
            ]),
            structures: HashMap::new(),
        }
    }
}

impl TypesRef {
    /// -> (size, is_primitive)
    pub fn get_size(&self, r#type: &str) -> Result<(usize, bool), ()> {
        if let Some(size) = self.primitives.get(r#type) {
            Ok((*size, true))
        } else if let Some((size, _)) = self.structures.get(r#type) {
            Ok((*size, false))
        } else {
            Err(())
        }
    }

    pub fn get_field_offset(&self, structure: &str, field: &str) -> Result<usize, ()> {
        let structure = self.structures.get(structure).ok_or(())?;
        structure.1.get(field).copied().ok_or(())
    }

    pub fn calculate_structure_size(
        &self,
        structure_name: &str,
        structure_fields: &HashMap<Box<str>, Box<str>>,
    ) -> Result<usize, Box<str>> {
        structure_fields.values().try_fold(0, |size, field_type| {
            match self.get_size(if *structure_name == **field_type {
                "ptr"
            } else {
                field_type
            }) {
                Ok((s, _)) => Ok(size + s),
                Err(_) => Err(field_type.clone()),
            }
        })
    }

    pub fn add_structure(
        &mut self,
        structure_name: &str,
        structure_fields: &HashMap<Box<str>, Box<str>>,
    ) -> Result<(), Box<str>> {
        self.structures.insert(
            structure_name.into(),
            (
                self.calculate_structure_size(structure_name, structure_fields)?,
                {
                    let mut offset = 0;
                    structure_fields
                        .iter()
                        .map(|(field_name, field_type)| {
                            match self.get_size(if *structure_name == **field_type {
                                "ptr"
                            } else {
                                field_type
                            }) {
                                Ok((size, is_primitive)) => Ok((field_name.clone(), {
                                    offset += if is_primitive { self.alignment } else { size };
                                    offset
                                })),
                                Err(_) => Err(field_type.clone()),
                            }
                        })
                        .collect::<Result<HashMap<_, _>, Box<str>>>()?
                },
            ),
        );

        Ok(())
    }
}

/// TODO: Probably move this to a generator
fn convert_structures(structures: HashMap<Box<str>, Node>) -> TypesRef {
    let mut tr = TypesRef::default();
    let st = structures.clone();
    let mut not_converted = st.keys().collect::<HashSet<_>>();

    let mut next = not_converted.iter().next().copied().cloned();
    while let Some(structure_name) = next.clone() {
        let structure_fields = structures.get(&structure_name).unwrap();
        if let Node::Structure { fields, .. } = structure_fields {
            if let Err(add_structure) = tr.add_structure(&structure_name, fields) {
                next = Some(add_structure);
            } else {
                not_converted.remove(&structure_name);
                next = not_converted.iter().next().copied().cloned();
            }
        }
    }

    tr
}

pub fn transform(
    Program {
        structures,
        functions,
    }: Program,
) -> Result<HashMap<Box<str>, Vec<Instruction>>, TransformerErrors> {
    let _ = convert_structures(structures);

    let mut functions_raw_body = HashMap::new();
    for (function_name, function_declaration) in functions {
        let (function_body, return_type, parameters) = if let Node::FunctionDeclaration {
            body,
            return_type,
            parameters,
            ..
        } = function_declaration
        {
            if let Node::Scope(function_body) = *body {
                (function_body, return_type, parameters)
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        };

        functions_raw_body.insert(function_name, (return_type, parameters, function_body));
    }
    let functions_raw_body = std::sync::Arc::new(functions_raw_body);

    let mut funcs = HashMap::new();
    for (function_name, (return_type, parameters, function_body)) in
        functions_raw_body.clone().iter()
    {
        let mut body = Vec::new();
        let mut temporaries = Temporaries::default();

        for (parameter_name, parameter_type) in parameters {
            temporaries.declare(parameter_name.clone(), parameter_type.clone());
        }

        for node in function_body.iter() {
            body.append(&mut transformers::transform(
                &functions_raw_body,
                node.clone(),
                return_type.clone(),
                &mut temporaries,
            )?)
        }

        dbg!(&temporaries);
        funcs.insert(function_name.clone(), body);
    }

    Ok(funcs)
}
