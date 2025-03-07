type Temporary = Box<str>;
type Type = Box<str>;
type Number = f64;
type FunctionName = Box<str>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ReturnNumber(Number),

    Allocate(Type, Temporary),

    FunctionCall(FunctionName, Box<[Temporary]>, Temporary),
    LoadArgument(usize, Temporary),

    ReturnVariable(Temporary),

    LoadNumber(Temporary, Number),

    Copy(Temporary, Temporary),

    AddVariables(Temporary, Temporary, Temporary),
    SubtractVariables(Temporary, Temporary, Temporary),
    MultiplyVariables(Temporary, Temporary, Temporary),
    DivideVariables(Temporary, Temporary, Temporary),
    ModuloVariables(Temporary, Temporary, Temporary),
}
