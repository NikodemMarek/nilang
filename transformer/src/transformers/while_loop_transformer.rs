use std::iter::once;

use nilang_types::{
    instructions::Instruction,
    nodes::{expressions::ExpressionNode, statements::StatementNode, Type},
};

use crate::{transform_body, transformers::transform_expression, Context, InstructionsIterator};

pub fn transform_while_loop<'a>(
    context @ Context {
        temporaries,
        labels,
        ..
    }: &'a Context,

    condition: ExpressionNode,
    body: &[StatementNode],
) -> InstructionsIterator<'a> {
    let condition_temporary = temporaries.declare(Type::Bool);
    let condition_instructions =
        transform_expression(context, condition, condition_temporary.clone(), &Type::Bool);

    let loop_label = labels.create();
    let end_loop_label = labels.create();

    Box::new(
        once(Ok(Instruction::Label(loop_label.clone())))
            .chain(once(Ok(Instruction::Declare(condition_temporary.clone()))))
            .chain(condition_instructions)
            .chain(once(Ok(Instruction::ConditionalJump(
                condition_temporary,
                end_loop_label.clone(),
            ))))
            .chain(transform_body(context, body, &Type::Void).collect::<Vec<_>>())
            .chain(once(Ok(Instruction::Jump(loop_label))))
            .chain(once(Ok(Instruction::Label(end_loop_label)))),
    )
}
