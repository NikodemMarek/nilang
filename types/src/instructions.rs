type Temporary = Box<str>;
type Number = f64;
type Char = char;
type Function = Box<str>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Declare(Temporary),

    FunctionCall(Function, Box<[Temporary]>, Option<Temporary>),
    TakeArgument(usize, Temporary),

    ReturnVariable(Temporary),

    LoadNumber(Temporary, Number),
    LoadChar(Temporary, Char),

    Copy(Temporary, Temporary),

    AddVariables(Temporary, Temporary, Temporary),
    SubtractVariables(Temporary, Temporary, Temporary),
    MultiplyVariables(Temporary, Temporary, Temporary),
    DivideVariables(Temporary, Temporary, Temporary),
    ModuloVariables(Temporary, Temporary, Temporary),
}
