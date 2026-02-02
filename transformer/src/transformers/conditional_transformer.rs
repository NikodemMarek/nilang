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

    Conditional {
        condition,
        body,
        chained,
    }: Conditional,
) -> InstructionsIterator<'a> {
    let condition_temporary = temporaries.declare(Type::Bool);
    let condition_instructions =
        transform_expression(context, condition, condition_temporary.clone(), &Type::Bool);

    let skip_if_label = labels.create();

    let base_conditional = once(Ok(Instruction::Declare(condition_temporary.clone())))
        .chain(condition_instructions)
        .chain(once(Ok(Instruction::ConditionalJump(
            condition_temporary,
            skip_if_label.clone(),
        ))))
        .chain(transform_body(context, &body, &Type::Void).collect::<Vec<_>>());

    if let Some(chained) = chained {
        let skip_else_label = labels.create();
        Box::new(
            base_conditional
                .chain(once(Ok(Instruction::Jump(skip_else_label.clone()))))
                .chain(once(Ok(Instruction::Label(skip_if_label))))
                .chain(transform_conditional(context, *chained))
                .chain(once(Ok(Instruction::Label(skip_else_label)))),
        )
    } else {
        Box::new(base_conditional.chain(once(Ok(Instruction::Label(skip_if_label)))))
    }
}
