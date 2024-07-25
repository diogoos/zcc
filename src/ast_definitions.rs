#[derive(Debug)]
pub enum Program<'a> {
    Function(FunctionDefinition<'a>),
    // Variable(VariableDefinition)
}

pub type FunctionDefinition<'a> = (&'a str, Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
    Return(Option<Expression>),
    // If(Expression, Box<Statement>, Option<Box<Statement>>)
}

#[derive(Debug)]
pub enum Expression {
    Int(String)
}
