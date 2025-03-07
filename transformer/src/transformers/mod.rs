mod function_call_transformer;
mod operation_transformer;
mod return_transformer;
mod variable_declaration_transformer;

use errors::TransformerErrors;

use nilang_types::nodes::{ExpressionNode, StatementNode};
use operation_transformer::transform_operation;
use return_transformer::transform_return;
use variable_declaration_transformer::transform_variable_declaration;

use crate::{temporaries::Temporaries, FunctionsRef, Instruction, TypesRef};

pub fn transform_statement(
    context: (&FunctionsRef, &TypesRef),
    node: StatementNode,
    return_type: Box<str>,
    temporaries: &mut Temporaries,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        StatementNode::Return(node) => transform_return(context, temporaries, *node, return_type),
        StatementNode::VariableDeclaration {
            name,
            r#type,
            value,
        } => transform_variable_declaration(context, temporaries, name, r#type, *value),
    }
}

pub fn transform_expression(
    context: (&FunctionsRef, &TypesRef),
    temporaries: &mut Temporaries,

    node: ExpressionNode,
    result_temporary_id: Box<str>,
) -> Result<Vec<Instruction>, TransformerErrors> {
    match node {
        ExpressionNode::Number(number) => {
            Ok(vec![Instruction::LoadNumber(result_temporary_id, number)])
        }
        ExpressionNode::VariableReference(variable) => {
            let variable_type = temporaries.type_of(&variable)?;
            if variable_type != "int" {
                return Ok(context
                    .1
                    .get_fields(variable_type)
                    .unwrap()
                    .iter()
                    .map(|(field, _)| {
                        let field_temporary =
                            <Box<str>>::from(format!("{}.{}", result_temporary_id, field));
                        temporaries.declare_named(field_temporary.clone(), "int".into());
                        Instruction::Copy(
                            field_temporary,
                            <Box<str>>::from(format!("{}.{}", variable, field)),
                        )
                    })
                    .collect::<Vec<_>>());
            }

            Ok(vec![Instruction::Copy(result_temporary_id, variable)])
        }
        ExpressionNode::FieldAccess { structure, field } => {
            let structure_temporary = temporaries.declare("int".into());
            let structure_instructions = transform_expression(
                context,
                temporaries,
                *structure,
                structure_temporary.clone(),
            )?;

            let field_temporary = <Box<str>>::from(format!("{}.{}", structure_temporary, field));

            Ok([
                structure_instructions,
                vec![Instruction::Copy(result_temporary_id, field_temporary)],
            ]
            .concat())
        }
        ExpressionNode::Operation { operator, a, b } => {
            transform_operation(context, temporaries, result_temporary_id, operator, *a, *b)
        }
        ExpressionNode::Object { r#type, fields } => {
            let mut instructions = Vec::new();
            for (field, value) in fields.iter() {
                let field_temp = <Box<str>>::from(format!("{}.{}", result_temporary_id, field));
                temporaries.declare_named(field_temp.clone(), r#type.clone());

                let mut field_instructions =
                    transform_expression(context, temporaries, value.clone(), field_temp)?;
                instructions.append(&mut field_instructions);
            }
            Ok(instructions)
        }
        ExpressionNode::FunctionCall { name, arguments } => todo!(),
    }
}
