use std::iter::once;

use errors::TransformerErrors;
use nilang_types::nodes::expressions::ExpressionNode;

use crate::{Context, InstructionsIterator};

use super::transform_expression;

pub fn transform_variable_assignment<'a>(
    context @ Context { temporaries, .. }: &'a Context,

    name: Box<str>,
    node: ExpressionNode,
) -> InstructionsIterator<'a> {
    let Ok(original_type) = temporaries.type_of(&name) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: name.clone(),
        })));
    };

    transform_expression(context, node, name, &original_type)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use nilang_types::{
        instructions::Instruction,
        nodes::{expressions::Primitive, Type},
    };

    use crate::{
        labels::Labels, structures_ref::tests::test_structures_ref, temporaries::Temporaries,
        FunctionsRef,
    };

    use super::*;

    #[test]
    fn test_variable_assignment() {
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            labels: Labels::default(),
            data: &RefCell::new(Vec::new()),
        };

        context.temporaries.declare_named("a".into(), Type::Int);

        assert_eq!(
            transform_variable_assignment(
                &context,
                "a".into(),
                ExpressionNode::Primitive(Primitive::Number(10.))
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            [Instruction::LoadNumber("a".into(), 10.)]
        );
    }
}
