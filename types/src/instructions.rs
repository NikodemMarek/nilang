type Temporary = Box<str>;
type Type = Box<str>;
type Number = f64;
type FunctionName = Box<str>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    LoadNumber(Number, Temporary),

    Copy(Temporary, Temporary),

    ReturnNumber(Number),
    ReturnVariable(Temporary),

    Allocate(Type, Temporary),

    FunctionCall(FunctionName, Box<[Temporary]>, Temporary),
    LoadArgument(usize, Temporary),

    AddNumbers(Number, Number, Temporary),
}
