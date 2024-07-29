// AST: Abstract Syntax Tree //
#![allow(dead_code)]

// A program consists of many top-level declarations
pub type Program = Vec<Declaration>;

// Top-level declarations can be functions,
// or (TODO) static variables/constants
#[derive(Debug)]
pub enum Declaration {
    Function(FunctionDefinition),
}

// Function consist of a name and multiple internal
// statements and (TODO) types
#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub statements: Vec<Statement>
}

// Statements called within functions -- this includes
// a return, or (TODO) a function call, or a variable
// declaration
#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression)
}

// Expressions are part of statements and can be 
// thought of as values -- for example, we return
// an expression, which could be `8` or `~1`, or `1 + 2`
#[derive(Debug, Clone)]
pub enum Expression {
    Constant(ConstantValue),
    Unary(UnaryExpressionType, Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Int(String)
}

// Unary expressions contained within statements
// and can be complements or negations
#[derive(Debug, Clone)]
pub enum UnaryExpressionType {
    Complement, Negation
}
