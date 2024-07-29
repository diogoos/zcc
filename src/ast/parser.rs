use super::symbols::*;
use crate::lexer::{Tag, Token};

macro_rules! syntax_error {
    ($msg:expr) => {
        return Err(ASTError::SyntaxError($msg.to_string()));
    };
    ($msg:expr, $($arg:tt)*) => {
        return Err(ASTError::SyntaxError(format!($msg, $($arg)*)));
    };
}


enum ASTParserState<'a> {
    Start,
    DeclarationKind(Tag),
    Declaration(Tag, &'a str)
}

enum FunctionParserState {
    Start,
    ArgumentListStart,
    ArgumentListEnd,
    Body,
    StatementEnd,
    End
}

pub struct ASTParser {
    buffer: String,
    tokens: Vec<Token>
}

#[derive(Debug)]
pub enum ASTError {
    SyntaxError(String)
}
impl std::fmt::Display for ASTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError(msg) => {
                return write!(f, "Syntax error: {}", msg);
            }
        }
    }
}

impl ASTParser {
    pub fn new(buffer: String, tokens: Vec<Token>) -> Self {
        Self { buffer, tokens }
    }

    pub fn parse(&mut self) -> Result<Program, ASTError> {
        use ASTParserState as S;

        let mut state = ASTParserState::Start;
        
        let mut index: usize = 0;
        let mut program: Program = vec![];

        let buffer = self.buffer.clone();
        let buffer = buffer.as_str();

        loop {
            if index >= self.tokens.len() {
                break Ok(program);
            }
            
            let token = &self.tokens[index];
            match state {
                S::Start => match token.tag {
                    Tag::KInt | Tag::KVoid => {
                        state = S::DeclarationKind(token.tag.clone())
                    },
                    Tag::Eof => {},
                    _ => {
                        syntax_error!("Unexpected token at {:?}: expected new declaration, got `{:?}` instead", token.range, token.tag);
                    }
                },

                S::DeclarationKind(kind) => match token.tag {
                    Tag::Identifier => {
                        let name = &buffer[token.range.clone()];
                        state = S::Declaration(kind, name);
                    },
                    _ => {
                        syntax_error!("Unexpected token at {:?}: expected declaration identifier, got `{:?}` instead", token.range, token.tag);
                    }
                },

                S::Declaration(_kind, name) => match token.tag {
                    // If we encounter a left parenthesis after a declaration,
                    // this means it is a function -- parse it and add it to the program
                    Tag::LParen => {
                        let (new_index, statements) = self.parse_function(index)?;
                        program.push(Declaration::Function(FunctionDefinition { name: name.to_string(), statements }));
                        index = new_index;

                        state = S::Start;
                        continue;
                    },

                    _ => {
                        syntax_error!("Unexpected token at {:?}: expected `(`, got `{:?}` instead", token.range, token.tag);
                    }
                }
            }

            index += 1;
        }
    }

    fn parse_function(&mut self, start_index: usize) -> Result<(usize, Vec<Statement>), ASTError> {
        use FunctionParserState as F;

        let mut index = start_index;
        let mut state = FunctionParserState::Start;

        let mut statements: Vec<Statement> = vec![];

        loop {
            if index >= self.tokens.len() {
                syntax_error!("Unexpected end of file while parsing function");
            }
            
            let token = &self.tokens[index];

            match state {
                F::Start => match token.tag {
                    Tag::LParen => {
                        state = F::ArgumentListStart;
                    }
                    _ => {
                        syntax_error!("Expected argument list following declaration at {:?}; got `{:?}` instead", token.range, token.tag);
                    }
                },

                F::ArgumentListStart => match token.tag {
                    Tag::KVoid => {},
                    Tag::RParen => {
                        state = F::ArgumentListEnd;
                    }
                    _ => {
                        syntax_error!("Unexpected token `{:?}` in argument list", token.tag);
                    }
                },

                F::ArgumentListEnd => match token.tag {
                    Tag::LBrace => {
                        state = F::Body;
                    },
                    _ => {
                        syntax_error!("Unexpected token `{:?}` after argument list", token.tag);
                    }
                },

                F::Body => match token.tag {
                    Tag::KReturn => {
                        index += 1;
                        let (new_index, expression) = self.parse_expression(index).expect("Unable to parse expression");
                        statements.push(Statement::Return(expression));
                        index = new_index;

                        state = F::StatementEnd;
                    },

                    Tag::RBrace => {
                        state = F::End;
                        continue;
                    }

                    _ => {
                        syntax_error!("Unexpected token `{:?}` in function body", token.tag);
                    }
                },

                F::StatementEnd => match token.tag {
                    Tag::Semicolon => {
                        state = F::Body;
                    },
                    _ => {
                        syntax_error!("Expected semicolon after expression -- found `{:?} instead", token.tag);
                    }
                },

                F::End => {
                    break Ok((index + 1, statements))
                }
            }

            index += 1;
        }
    }

    fn parse_expression(&mut self, index: usize) -> Result<(usize, Expression), ASTError> {
        let mut index = index;

        loop {
            if index >= self.tokens.len() {
                syntax_error!("Unexpected end of file while parsing expression");
            }
            
            let token = &self.tokens[index];

            match token.tag {
                Tag::NumberLiteral => {
                    let value = &self.buffer[token.range.clone()];
                    break Ok((index, Expression::Constant(ConstantValue::Int(value.to_string()))));
                },

                // Unary operators and their sub expressions
                Tag::OpNegation | Tag::OpComplement => {
                    let unary_type = match token.tag {
                        Tag::OpNegation => UnaryExpressionType::Negation,
                        Tag::OpComplement => UnaryExpressionType::Complement,
                        _ => panic!("Internal parser error -- unary type undefined"),
                    };

                    index += 1;
                    let (new_index, subexpression) = self.parse_expression(index)?;
                    index = new_index;
                    return Ok((index, Expression::Unary(unary_type, Box::new(subexpression))));
                },

                Tag::LParen => {
                    index += 1;
                    let (shift, expression) = self.parse_expression(index)?;
                    index = shift + 1;
                    assert_eq!(self.tokens[index].tag, Tag::RParen);
                    return Ok((index, expression));
                }

                _ => {
                    syntax_error!("Unexpected token `{:?}` in expression", token.tag);
                }
            }
        }
    }
}


#[cfg(test)]
#[path = "./test.rs"]
mod ast_test;
