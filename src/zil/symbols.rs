// zil : Z intermediate language
pub type Program = Vec<Construct>;

pub enum Construct {
    Function(FunctionDefinition)
}

pub struct FunctionDefinition {
    pub identifier: String,
    pub instructions: Vec<Instruction>
}

pub enum Instruction {
    Return(Value),
    Unary(UnaryInstructionDefinition)
}

pub struct UnaryInstructionDefinition {
    pub operator: UnaryInstructionOperator,
    pub source: Value,
    pub destination: Value
}

pub enum UnaryInstructionOperator {
    Complement,
    Negate
}

#[derive(Clone)]
pub enum Value {
    Constant(String),
    Variable(String)
}
