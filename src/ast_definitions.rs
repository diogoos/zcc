use crate::lexer::*;

#[derive(Debug)]
pub enum FunctionDefinition<'a> {
    Function(&'a str, Statement)
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    // If(Expression, Box<Statement>, Option<Box<Statement>>)
}

#[derive(Debug)]
pub enum Expression {
    Int(String)
}

pub fn parse_program(buffer: &String, tokens: Vec<Token>) -> FunctionDefinition {
    // let mut tokens = tokens.clone();
    let mut iterator = tokens.iter().cloned();
    
    // The first token in any program should be a keyword (int, void)
    assert!(KEYWORDS.contains(&iterator.next().unwrap().tag));
    
    // The next token should be an identifier with the function name;
    let name_token = iterator.next().unwrap();
    assert!(name_token.tag == Tag::Identifier);
    let name = &buffer[name_token.range.clone()];

    // TODO
    // Then, we should parse the arguments

    // Find the beginning of the statement, indicated by left bracket
    // println!("{:#?}", iterator);
    _ = iterator.by_ref().take_while(|x| !matches!(x.tag, Tag::LBrace)).collect::<Vec<_>>();
    let statement_tokens = iterator.by_ref().take_while(|x| !matches!(x.tag, Tag::RBrace)).collect();
    // println!("{:#?}", statement_tokens);

    let statement = parse_statement(buffer, statement_tokens);

    return FunctionDefinition::Function(name, statement);
}

fn parse_statement(buffer: &String, tokens: Vec<Token>) -> Statement {
    let mut iter = tokens.iter().cloned();

    // right now, the only statement is return, so expect it
    // println!("{:?}", iter.next().unwrap());
    assert!(iter.next().unwrap().tag == Tag::KReturn);

    // parse the remainder of the expression (until semicolon)
    let expression_tokens = iter.by_ref().take_while(|t| !matches!(t.tag, Tag::Semicolon)).collect();
    let expression: Expression = parse_expression(buffer, expression_tokens);

    return Statement::Return(expression);
}

fn parse_expression(buffer: &String, tokens: Vec<Token>) -> Expression {
    // The only expression we have is a number literal
    assert!(tokens[0].tag == Tag::NumberLiteral);
    let expression_value = &buffer[tokens[0].range.clone()];
    return Expression::Int(expression_value.to_string());
}

// fn ast_example() {
//     let statment = Statement::Return(Expression::Int("2".to_string()));
//     let program = FunctionDefinition::Function("main".to_string(), statment);

//     // Function (
//     //     "main"
//     //     Return (
//     //         Int ( 2 )
//     //     )
//     // )

//     // Kint identifier lparen
// }