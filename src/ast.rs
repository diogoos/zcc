use crate::ast_definitions::*;
use crate::lexer::{Tag, Token};

enum ASTParserState {
    Start,
    DeclarationStart,
    DeclarationValue,
    FunctionArguments,
    FunctionArgumentsEnded,
    StatementBody,
    ExpressionBody,
    ExpressionEnd
}

struct ASTStore<'a> {
    statement_block: Vec<(Token, Vec<Token>)>, // [ (Identifier Token, [Expression, Expression])]

    current_statement_token: Option<Token>,
    expression_block: Vec<Token>,

    current_identifier: Option<&'a str>,
    current_declaration_type: Option<Tag>,
}
impl<'a> ASTStore<'a> {
    pub fn empty() -> Self {
        Self {
            statement_block: vec![],

            current_statement_token: None,
            expression_block: vec![],
            
            current_identifier: None,
            current_declaration_type: None
        }
    }

    pub fn close_statement_block(&mut self) {
        // There's probably a better way of donig this without cloning...
        let statement_token = self.current_statement_token.clone().expect("Internal parser storage error: attempted to close statement block without an identifying token");
        self.current_statement_token = None;
        let expression_block = self.expression_block.clone();
        self.expression_block = vec![];

        // Push the block to be dealt with
        self.statement_block.push((statement_token, expression_block))
    }
}

pub struct ASTParser<'a> {
    buffer: String,
    tokens: Vec<Token>,
    storage: ASTStore<'a>,

    index: usize,
    is_parsing_function: bool
}

pub enum ASTError {
    SyntaxError(String),
    InternalParserError(String)
}

impl<'a> ASTParser<'a> {
    pub fn new(buffer: String, tokens: Vec<Token>) -> Self {
        Self { buffer, tokens,  storage: ASTStore::empty(), index: 0, is_parsing_function: false }
    }

    pub fn parse(&'a mut self) -> Result<Vec<Program<'a>>, ASTError> {
        use ASTParserState as S;

        let mut state = ASTParserState::Start;
        let mut result: Vec<Program> = vec![];

        loop {
            if self.index >= self.tokens.len() {
                if self.is_parsing_function {   
                    let function_name: &str = self.storage.current_identifier.unwrap_or("unknown");
                    return Err(ASTError::SyntaxError(format!("Unexpected end of file while parsing function `{}`", function_name)))
                }                

                // finished processing all tokens,
                // return the current program
                break Ok(result);
            }

            let token = &self.tokens[self.index];
            match state {
                S::Start => match token.tag {
                    Tag::KInt | Tag::KVoid => {
                        state = S::DeclarationStart;
                        self.storage.current_declaration_type = Some(token.tag.clone());
                    },
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected Type keyword, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::DeclarationStart => match token.tag {
                    Tag::Identifier => {
                        state = S::DeclarationValue;

                        let str = &self.buffer[token.range.clone()];
                        self.storage.current_identifier = Some(str);
                    },
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected identifier, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::DeclarationValue => match token.tag {
                    // if what follows the token declaration is a parenthesis,
                    // declare a new function
                    Tag::LParen => {
                        state = S::FunctionArguments
                    }

                    // if what follows is an equal sign, create new variable
                    // TODO

                    // otherwise, error
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected `(`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::FunctionArguments => match token.tag {
                    // TODO: parse arguments in form of:
                    // `int x, int y`
                    // (KInt Identifier) Tag::Comma (KInt Identifier)
                    Tag::KInt | Tag::KVoid | Tag::Identifier => { },
                    

                    // After all arguments have been declared, only a right (closing) parenthesis can follow
                    Tag::RParen => {
                        state = S::FunctionArgumentsEnded;
                    },
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected function arguments or `)`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                // After a function's arguments have been declared, only a left brace can follow
                S::FunctionArgumentsEnded => match token.tag {
                    Tag::LBrace => {
                        state = S::StatementBody;
                        self.is_parsing_function = true;
                    },
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected `)`, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                // Parse the contents of a function, ie. statements
                // Right now, the only statement we support is `return`
                S::StatementBody => match token.tag {
                    Tag::KReturn => {
                        state = S::ExpressionBody;
                        self.storage.current_statement_token = Some(token.clone());
                    },
                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected statement, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                S::ExpressionBody => match token.tag {
                    // Right now, the only expression we support is a Number Literal
                    // When we encounter it, store it in the collected current expression
                    Tag::NumberLiteral => {
                        self.storage.expression_block.push(token.clone());
                    },
                    
                    // When encountering a semicolon in an expression, finish the collection
                    // and move to ExpressionEnd
                    Tag::Semicolon => {
                        state = S::ExpressionEnd
                    },

                    _ => {
                        return Err(ASTError::SyntaxError(format!("Unexpected token at {:?}: expected expression, got `{:?}` instead", token.range, token.tag)));
                    }
                },

                // After an expression has ended, we can have:
                // (a) another statement inside the current function
                // (b) another statement in the global context
                // (c) the end of the current function, if it exists
                S::ExpressionEnd => {
                    // at the end of an expression, firstly always close the current block
                    self.storage.close_statement_block();

                    match token.tag {
                        Tag::RBrace => { // case (c)
                            self.is_parsing_function = false;
                            
                            // add the current function to the program
                            let function_name = match self.storage.current_identifier {
                                Some(val) => val,
                                None => {
                                    return Err(ASTError::InternalParserError(format!("Unable to get storage.current_identifier at S::ExpressionEnd Tag::RBrace")))
                                }
                            };

                            // map the saved statement tokens into AST statements
                            let mut statements: Vec<Statement> = vec![];
                            for (statement_type, expression_block) in &self.storage.statement_block {
                                // right now, we only support one 0 or 1 expressions (nothing, or numerical)
                                let mut expression_value: Option<Expression> = None;
                                assert!(expression_block.len() < 2);
                                if expression_block.len() == 1 {
                                    match expression_block[0].tag {
                                        Tag::NumberLiteral => {
                                            let number_str = &self.buffer[expression_block[0].range.clone()];
                                            expression_value = Some(Expression::Int(number_str.to_string()));
                                        },

                                        _ => {
                                            return Err(ASTError::InternalParserError(format!("Unable to parse expression of type `{:?}`", expression_block[0].tag)))
                                        }
                                    }
                                }

                                // right now, we only support the return statement
                                match statement_type.tag {
                                    Tag::KReturn => {
                                        statements.push(Statement::Return(expression_value));
                                    },
                                    
                                    _ => {
                                        return Err(ASTError::InternalParserError(format!("Unable to map statement of type `{:?}`", statement_type.tag)))
                                    }
                                }
                            }

                            // Reset statement block
                            self.storage.statement_block = vec![];

                            // create a function with the given name and statements
                            let function_def: FunctionDefinition = (function_name, statements);
                            result.push(Program::Function(function_def));

                            // start parsing all over again
                            state = S::Start;
                        }

                        _ => {
                            if self.is_parsing_function { // if we are still within a function, case (a)
                                state = S::StatementBody;
                            } else { // if we are outside a function, case (c)
                                state = S::Start;
                            }

                            // in case we are starting the parsing over,
                            // we need to re-evaluate the current token, so continue
                            // without updating the index
                            continue; 
                        }
                    }
                }
            }

            self.index += 1;
        }
    }
}
