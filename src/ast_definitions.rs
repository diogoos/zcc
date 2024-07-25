pub enum Program<'a> {
    Function(FunctionDefinition<'a>),
    Dud
}

pub type FunctionDefinition<'a> = (&'a str, Statement);

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    // If(Expression, Box<Statement>, Option<Box<Statement>>)
}

#[derive(Debug)]
pub enum Expression {
    Int(String)
}
