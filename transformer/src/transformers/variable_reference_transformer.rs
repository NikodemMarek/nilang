use std::iter::once;

use errors::TransformerErrors;

use crate::{structures_ref::copy_all_fields, Context, InstructionsIterator, Type};

pub fn transform_variable_reference<'a>(
    Context {
        structures,
        temporaries,
        ..
    }: &'a Context,

    variable: Box<str>,
    result: Box<str>,
    r#type: &Type,
) -> InstructionsIterator<'a> {
    let Ok(source_type) = temporaries.type_of(&variable) else {
        return Box::new(once(Err(TransformerErrors::TemporaryNotFound {
            name: variable.clone(),
        })));
    };

    if *r#type != source_type {
        return Box::new(once(Err(TransformerErrors::TypeMismatch {
            expected: r#type.clone(),
            found: source_type.clone(),
        })));
    }

    copy_all_fields(structures, temporaries, variable, result, &source_type)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use nilang_types::instructions::Instruction;

    use crate::{
        structures_ref::tests::test_structures_ref, temporaries::Temporaries,
        transformers::variable_reference_transformer::transform_variable_reference, Context,
        FunctionsRef, Type,
    };

    #[test]
    fn test_transform_variable_reference() {
        let context = Context {
            functions: &FunctionsRef::default(),
            structures: &test_structures_ref(),
            temporaries: Temporaries::default(),
            data: &RefCell::new(Vec::new()),
        };

        context
            .temporaries
            .declare_named("original".into(), Type::Object("Rect".into()));
        context
            .temporaries
            .declare_named("copy".into(), Type::Object("Rect".into()));

        assert_eq!(
            transform_variable_reference(
                &context,
                "original".into(),
                "copy".into(),
                &Type::Object("Rect".into()),
            )
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
            vec![
                Instruction::Declare("copy.end.x".into()),
                Instruction::Copy("copy.end.x".into(), "original.end.x".into()),
                Instruction::Declare("copy.end.y".into()),
                Instruction::Copy("copy.end.y".into(), "original.end.y".into()),
                Instruction::Declare("copy.start.x".into()),
                Instruction::Copy("copy.start.x".into(), "original.start.x".into()),
                Instruction::Declare("copy.start.y".into()),
                Instruction::Copy("copy.start.y".into(), "original.start.y".into()),
            ],
        );
    }
}
