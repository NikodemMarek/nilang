type Temporary = Box<str>;
type Number = f64;
type FunctionName = Box<str>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Declare(Temporary),

    FunctionCall(FunctionName, Box<[Temporary]>, Option<Temporary>),
    LoadArgument(usize, Temporary),

    ReturnVariable(Temporary),

    LoadNumber(Temporary, Number),
    LoadChar(Temporary, char),

    Copy(Temporary, Temporary),

    AddVariables(Temporary, Temporary, Temporary),
    SubtractVariables(Temporary, Temporary, Temporary),
    MultiplyVariables(Temporary, Temporary, Temporary),
    DivideVariables(Temporary, Temporary, Temporary),
    ModuloVariables(Temporary, Temporary, Temporary),
}
