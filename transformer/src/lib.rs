mod functions_ref;
mod structures_ref;
mod temporaries;
mod transformers;

use std::iter::once;

use errors::TransformerErrors;
pub use functions_ref::FunctionsRef;
use nilang_types::{
    instructions::Instruction,
    nodes::{FunctionDeclaration, Parameter, StatementNode, Type},
};
pub use structures_ref::StructuresRef;
use temporaries::Temporaries;

type InstructionsIterator<'a> =
    Box<dyn Iterator<Item = Result<Instruction, TransformerErrors>> + 'a>;

pub fn transform_function<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    FunctionDeclaration {
        body,
        return_type,
        parameters,
        ..
    }: &'a FunctionDeclaration,
) -> InstructionsIterator<'a> {
    let temporaries = Temporaries::default();

    let parameters = transform_parameters(
        &context.1,
        &temporaries,
        parameters
            .iter()
            .map(|(name, r#type)| (name.clone(), r#type.clone()))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    let body = transform_body(context, &temporaries, body, return_type);

    Box::new(parameters.chain(body).collect::<Vec<_>>().into_iter())
}

fn transform_body<'a>(
    context: &'a (FunctionsRef, StructuresRef),
    temporaries: &'a Temporaries,
    body: &'a [StatementNode],
    return_type: &'a Type,
) -> InstructionsIterator<'a> {
    Box::new(body.iter().flat_map(|node| {
        transformers::transform_statement(context, node.clone(), return_type, temporaries)
    }))
}

fn transform_parameters<'a>(
    context: &StructuresRef,
    temporaries: &Temporaries,
    parameters: &[Parameter],
) -> InstructionsIterator<'a> {
    let mut instructions = Vec::new();
    let mut i = 0;
    for (parameter_name, parameter_type) in parameters.iter() {
        let parameter_type = parameter_type.clone();
        if let Type::Object(object_type) = &parameter_type {
            let object_fields_recursive = match context.get_fields_flattened(object_type) {
                Ok(object_fields_recursive) => object_fields_recursive,
                Err(e) => return Box::new(once(Err(e))),
            };

            for (field, field_type) in object_fields_recursive {
                let field = Into::<Box<str>>::into(format!("{}.{}", parameter_name, field));
                temporaries.declare_named(field.clone(), field_type.to_owned());
                instructions.push(Ok(Instruction::TakeArgument(i, field.clone())));
                i += 1;
            }
        } else {
            temporaries.declare_named(parameter_name.clone(), parameter_type);
            instructions.push(Ok(Instruction::TakeArgument(i, parameter_name.clone())));
            i += 1;
        }
    }
    Box::new(instructions.into_iter())
}
