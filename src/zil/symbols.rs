// zil : Z intermediate language
pub type Program = Vec<Construct>;

pub enum Construct {
    Function(FunctionDefinition)
}

pub struct FunctionDefinition {
    identifier: String,
    instructions: Vec<Instruction>
}

pub enum Instruction {
    Return(Value),
    Unary(UnaryInstructionDefinition)
}

pub struct UnaryInstructionDefinition {
    operator: UnaryInstructionOperator,
    source: Value,
    destination: Value
}

pub enum UnaryInstructionOperator {
    Complement,
    Negate
}

pub enum Value {
    Constant(String),
    Variable(String)
}
