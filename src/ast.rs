use crate::ast_definitions::*;
use crate::lexer::{Tag, Token};

enum ASTParserState {
    Start,
    SawType,
    SawIdentifier,
    FunctionArguments,
    FunctionArgumentsEnded,
    StatementBody,
    ExpressionBody,
    ExpressionBodyEnd,
    ExpressionEnd
}

pub struct ASTParser {
    buffer: String,
    tokens: Vec<Token>,

    index: usize,
    function_depth: i8
}

pub enum ParserError {
    UnexpectedToken(String),
    UnexpectedEof(String),
    Unknown
}

impl ASTParser {
    pub fn new(buffer: String, tokens: Vec<Token>) -> Self {
        Self { buffer, tokens, index: 0, function_depth: 0 }
    }

    pub fn parse<'a>(&mut self) -> Result<Program<'a>, ParserError> {
        use ASTParserState as S;

        let mut state = ASTParserState::Start;

        loop {
            if self.index >= self.tokens.len() {

                if self.function_depth != 0 {
                    return Err(ParserError::UnexpectedEof(format!("Unexpected end of file, missing a closing bracket")))
                }
                // println!("Exceeded token completion time!");
                // return Err(ParserError::Unknown);
                break Ok(Program::Dud);
            }

            let token = &self.tokens[self.index];
            match state {
                S::Start => match token.tag {
                    Tag::KInt | Tag::KVoid => {
                        state = S::SawType;
                    },
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected Type keyword, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::SawType => match token.tag {
                    Tag::Identifier => {
                        state = S::SawIdentifier
                    },
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected identifier, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::SawIdentifier => match token.tag {
                    // if parenthesis, new function
                    Tag::LParen => {
                        state = S::FunctionArguments
                    }
                    // if equal sign, new variable
                    // TODO

                    // otherwise, error
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected `(`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::FunctionArguments => match token.tag {
                    Tag::KInt | Tag::KVoid | Tag::Identifier => { },
                    Tag::RParen => {
                        state = S::FunctionArgumentsEnded;
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected function arguments or `)`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::FunctionArgumentsEnded => match token.tag {
                    Tag::LBrace => {
                        state = S::StatementBody;
                        self.function_depth += 1;
                    },
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected `)`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::StatementBody => match token.tag {
                    // right now, the only statement we support is `return`
                    Tag::KReturn => {
                        state = S::ExpressionBody;
                    },
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected statement, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::ExpressionBody => match token.tag {
                    // right now, the only expression we support is a Number Literal
                    Tag::NumberLiteral => {
                        state = S::ExpressionBodyEnd
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected expression, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::ExpressionBodyEnd => match token.tag {
                    // an expression body must always end with a semicolon
                    Tag::Semicolon => {
                        state = S::ExpressionEnd
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: expected semicolon, got `{:?}` instead", token.range, token.tag)));   
                    }
                },

                // after an expression, we can have:
                // (a) another expression
                // (b) the end of a function
                S::ExpressionEnd => match token.tag {
                    Tag::RBrace => { // (b)
                        self.function_depth -= 1;
                        if self.function_depth < 0 {
                            return Err(ParserError::UnexpectedToken(format!("Unexpected token at {:?}: {:?}", token.range, token.tag)));   
                        }

                        state = S::Start;
                    }

                    _ => { // (a)
                        if self.function_depth > 0 {
                            state = S::StatementBody;
                        } else {
                            state = S::Start;
                        }
                        continue;
                    }
                }
            }

            self.index += 1;
        }
    }
}
