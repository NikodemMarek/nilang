use std::iter::once;

use nilang_types::{
    instructions::Instruction,
    nodes::{Conditional, Type},
};

use crate::{transform_body, transformers::transform_expression, Context, InstructionsIterator};

pub fn transform_conditional<'a>(
    context @ Context {
        temporaries,
        labels,
        ..
    }: &'a Context,

    Conditional { condition, body }: Conditional,
) -> InstructionsIterator<'a> {
    let condition_temporary = temporaries.declare(Type::Bool);
    let condition_instructions =
        transform_expression(context, condition, condition_temporary.clone(), &Type::Bool);

    let label = labels.create();
    Box::new(
        once(Ok(Instruction::Declare(condition_temporary.clone())))
            .chain(condition_instructions)
            .chain(once(Ok(Instruction::ConditionalJump(
                condition_temporary,
                label.clone(),
            ))))
            .chain(transform_body(context, &body, &Type::Void).collect::<Vec<_>>())
            .chain(once(Ok(Instruction::Label(label)))),
    )
}
