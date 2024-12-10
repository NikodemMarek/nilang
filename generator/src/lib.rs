#![feature(box_patterns)]

mod flavour;
mod flavours;
mod generators;

mod to_assembly;
mod transformers;
mod utils;

use std::collections::HashMap;

use errors::GeneratorErrors;
use flavours::{gnu_flavour::GnuFlavour, x86_registers::Gnu64Registers};
use nilang_types::{instructions::Instruction, nodes::Node};
use utils::generate_function;

/// (size, <field, offset>)
type StructSize = (u8, Option<HashMap<Box<str>, u8>>);

/// <type, (size, <field, offset>)>
#[derive(Debug)]
struct TypesRef(HashMap<Box<str>, StructSize>);

impl Default for TypesRef {
    fn default() -> Self {
        Self(HashMap::from([
            ("ptr".into(), (4, None)),
            ("int".into(), (4, None)),
            ("bool".into(), (1, None)),
            ("char".into(), (1, None)),
        ]))
    }
}

impl TypesRef {
    pub fn get_structure_size(&self, r#type: &str) -> Result<u8, GeneratorErrors> {
        match self.0.get(r#type) {
            Some(size) => Ok(size.0),
            None => Err(GeneratorErrors::StructureNotDefined {
                name: r#type.into(),
            }),
        }
    }

    pub fn get_field_offset(&self, r#type: &str, field: &str) -> Result<u8, GeneratorErrors> {
        let structure = match self.0.get(r#type) {
            Some((_, Some(structure))) => structure,
            Some(_) => unreachable!(),
            None => Err(GeneratorErrors::StructureNotDefined {
                name: r#type.into(),
            })?,
        };
        match structure.get(field) {
            Some(offset) => Ok(*offset),
            None => Err(GeneratorErrors::FieldNotDefined { name: field.into() }),
        }
    }
}

fn calculate_structure_size(tsr: &TypesRef, structure: &Node) -> Result<u8, GeneratorErrors> {
    if let Node::Structure { fields, name } = structure {
        fields.iter().try_fold(0, |size, (_, r#type)| {
            if r#type == name {
                todo!("probably treat it as a pointer")
            }
            tsr.0.get(r#type).map(|s| size + s.0).ok_or_else(|| {
                GeneratorErrors::StructureNotDefined {
                    name: r#type.to_owned(),
                }
            })
        })
    } else {
        unreachable!()
    }
}

pub fn generate(functions: HashMap<Box<str>, Vec<Instruction>>) -> eyre::Result<String> {
    let mut flavour = GnuFlavour::<Gnu64Registers>::default();
    let mut code = Vec::new();

    for (name, instructions) in functions.into_iter() {
        let mut function = generate_function(
            &name,
            &generators::generate(&mut flavour, &mut instructions.into_iter())?
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );

        code.append(&mut function);
    }

    Ok(generate_program(&[], &code))
}

fn generate_program(data: &[String], code: &[String]) -> String {
    let start_fn = generate_function(
        "start",
        &[
            String::from("call _main"),
            String::from("movl $1, %eax"),
            // String::from("movl $0, %ebx"),
            String::from("int $0x80"),
        ],
    );

    format!(
        ".data\n{}\n.text\n{}",
        &data.join("\n"),
        &[start_fn, code.to_vec()].concat().join("\n")
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nilang_types::nodes::{Node, Operator, Program};

    use crate::{generate, generate_program};

    #[test]
    fn test_generate() {
        let node = Program {
            structures: HashMap::new(),
            functions: HashMap::from([(
                "main".into(),
                Node::FunctionDeclaration {
                    name: "main".into(),
                    parameters: [].into(),
                    return_type: "int".into(),
                    body: Box::new(Node::Scope(Vec::from(&[Node::Return(Box::new(
                        Node::Operation {
                            operator: Operator::Add,
                            a: Box::new(Node::Number(1.)),
                            b: Box::new(Node::Number(2.)),
                        },
                    ))]))),
                },
            )]),
        };
        let output = generate(node);

        assert_eq!(
            output.unwrap(),
            ".data\n\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    int $0x80\n    ret\n\n.globl _main\n_main:\n    pushq %rbp\n    movq %rsp, %rbp\n    movq $1, %rbx\n    add $2, %rbx\n    leave\n    ret\n"
        )
    }

    #[test]
    fn test_generate_program() {
        assert_eq!(
            generate_program(
                &Vec::from([String::from("data")]),
                &Vec::from([String::from("code")]),
            ),
            ".data\ndata\n.text\n.globl _start\n_start:\n    call _main\n    movl $1, %eax\n    int $0x80\n    ret\n\ncode"
        );
    }
}
