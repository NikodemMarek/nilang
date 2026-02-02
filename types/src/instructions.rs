type Temporary = Box<str>;
type Label = Box<str>;
type Boolean = bool;
type Number = f64;
type Char = char;
type Function = Box<str>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Label(Label),
    ConditionalJump(Temporary, Label),

    Declare(Temporary),

    FunctionCall(Function, Box<[Temporary]>, Option<Temporary>),
    TakeArgument(usize, Temporary),

    ReturnVariable(Temporary),

    LoadBoolean(Temporary, Boolean),
    LoadNumber(Temporary, Number),
    LoadChar(Temporary, Char),
    LoadStringLocation(Temporary, Temporary),

    Copy(Temporary, Temporary),

    AddVariables(Temporary, Temporary, Temporary),
    SubtractVariables(Temporary, Temporary, Temporary),
    MultiplyVariables(Temporary, Temporary, Temporary),
    DivideVariables(Temporary, Temporary, Temporary),
    ModuloVariables(Temporary, Temporary, Temporary),
}
