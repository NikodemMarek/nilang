mod functions_ref;
mod structures_ref;
mod temporaries;
mod transformers;

use std::{cell::RefCell, iter::once};

use errors::NilangError;
pub use functions_ref::FunctionsRef;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, StatementNode, Type, TypedIdentifier},
    Localizable,
};
pub use structures_ref::StructuresRef;
use temporaries::Temporaries;

type InstructionsIterator<'a> = Box<dyn Iterator<Item = Result<Instruction, NilangError>> + 'a>;

type Declaration = (Box<str>, Box<str>);
type Data = RefCell<Vec<Declaration>>;

struct Context<'a> {
    functions: &'a FunctionsRef,
    structures: &'a StructuresRef,
    temporaries: Temporaries,
    data: &'a Data,
}

pub fn transform_function<'a>(
    refs: &'a (FunctionsRef, StructuresRef),
    FunctionDeclaration {
        body,
        return_type,
        parameters,
        ..
    }: &'a FunctionDeclaration,
) -> (InstructionsIterator<'a>, Vec<Declaration>) {
    let temporaries = Temporaries::default();

    let parameters = transform_parameters(
        &refs.1,
        &temporaries,
        parameters
            .iter()
            .map(|(name, r#type)| (name.clone(), r#type.clone()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let data: Data = RefCell::new(Vec::new());
    let context = Context {
        functions: &refs.0,
        structures: &refs.1,
        temporaries,
        data: &data,
    };

    let body = transform_body(&context, body, return_type);

    (
        Box::new(parameters.chain(body).collect::<Vec<_>>().into_iter()),
        data.take(),
    )
}

fn transform_body<'a>(
    context: &'a Context,

    body: &'a [Localizable<StatementNode>],
    return_type: &'a Localizable<Type>,
) -> InstructionsIterator<'a> {
    Box::new(body.iter().flat_map(move |node| {
        transformers::transform_statement(context, node.clone(), return_type)
    }))
}

fn transform_parameters<'a>(
    context: &StructuresRef,
    temporaries: &Temporaries,
    parameters: &[TypedIdentifier],
) -> InstructionsIterator<'a> {
    let mut instructions = Vec::new();
    let mut i = 0;
    for (parameter_name, parameter_type) in parameters.iter() {
        let parameter_type = parameter_type.clone();
        if let Type::Object(object_type) = &*parameter_type {
            let Some(object_fields_recursive) = context.get_fields_flattened(object_type) else {
                return Box::new(once(Err(NilangError {
                    location: parameter_name.location,
                    error: errors::TransformerErrors::TypeNotFound(object_type.clone()).into(),
                })));
            };

            for (field, field_type) in object_fields_recursive {
                let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name.object, field));
                temporaries.declare_named(field.clone(), field_type.to_owned());
                instructions.push(Ok(Instruction::TakeArgument(i, field.clone())));
                i += 1;
            }
        } else {
            temporaries.declare_named((**parameter_name).clone(), (*parameter_type).clone());
            instructions.push(Ok(Instruction::TakeArgument(i, (**parameter_name).clone())));
            i += 1;
        }
    }
    Box::new(instructions.into_iter())
}
