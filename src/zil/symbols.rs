// zil : Z intermediate language
pub type Program = Vec<Construct>;

#[derive(Debug)]
pub enum Construct {
    Function(FunctionDefinition)
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub instructions: Vec<Instruction>
}

#[derive(Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryInstructionDefinition)
}

#[derive(Debug)]
pub struct UnaryInstructionDefinition {
    pub operator: UnaryInstructionOperator,
    pub source: Value,
    pub destination: Value
}

#[derive(Debug)]
pub enum UnaryInstructionOperator {
    Complement,
    Negate
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Value {
    Constant(String),
    Variable(String)
}
