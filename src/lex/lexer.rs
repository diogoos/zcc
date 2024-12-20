use std::{fs, ops::Range, path::PathBuf};
use phf::phf_map;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Tag {
    Invalid,

    Eof,
    Identifier,
    
    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon,
    NumberLiteral,

    // Keywords:
    KInt,
    KVoid,
    KReturn,

    // Operations
    OpNegation,
    OpComplement
}

static TOKEN_KEYWORDS: phf::Map<&'static str, Tag> = phf_map! {
    "int" => Tag::KInt,
    "void" => Tag::KVoid,
    "return" => Tag::KReturn,
};
impl Tag {
    fn get_keyword(key: &str) -> Option<Tag> {
        return TOKEN_KEYWORDS.get(key).cloned();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub tag: Tag,
    pub range: Range<usize>
}

pub struct Lexer {
    pub buffer: String,
    index: usize
}

enum TokenizerState {
    Start,
    Identifier,
    Int
}

impl Lexer {
    #[cfg(test)]
    pub fn load_test_str(str: &str) -> Self {
        return Self {
            buffer: str.to_string(),
            index: 0
        }
    }

    pub fn load(path: &PathBuf) -> Self {
        let buffer = fs::read_to_string(path).expect("Error: unable to read preprocessed file");
        return Self {
            buffer,
            index: 0
        }
    }

    pub fn next(&mut self) -> Token {
        use TokenizerState as S;
        use Tag as T;

        let mut state: TokenizerState = S::Start;
        let mut result = Token {
            tag: T::Eof,
            range: Range {
                start: self.index,
                end: 0
            }
        };

        let max_length = self.buffer.len();
        loop {
            if self.index == max_length { self.index += 1; break };
            let c = self.buffer.chars().nth(self.index).expect("Lexer advanced past EOF!");
            
            match state {
                S::Start => match c {
                    // skip whitespace/newlines
                    ' ' | '\n' | '\t' | '\r' => {
                        result.range.start += 1;
                    },

                    // match an identifier and enter that state
                    'a'..='z' | 'A'..='Z' | '_' => {
                        state = S::Identifier;
                        result.tag = T::Identifier;
                    },


                    // match a parenthesis/brace and return that token
                    '(' => {
                        result.tag = T::LParen;
                        self.index += 1;
                        break;
                    },
                    ')' => {
                        result.tag = T::RParen;
                        self.index += 1;
                        break;
                    }
                    '{' => {
                        result.tag = T::LBrace;
                        self.index += 1;
                        break;
                    },
                    '}' => {
                        result.tag = T::RBrace;
                        self.index += 1;
                        break;
                    },

                    // match semicolon and return that token directly
                    ';' => {
                        result.tag = T::Semicolon;
                        self.index += 1;
                        break;
                    },

                    // enter integer matching mode
                    '0'..='9' => {
                        state = S::Int;
                        result.tag = T::NumberLiteral;
                    },

                    '-' => {
                        result.tag = T::OpNegation;
                        self.index += 1;
                        break;
                    },

                    '~' => {
                        result.tag = T::OpComplement;
                        self.index += 1;
                        break;
                    },

                    // we encountered an invalid token -- return it directly
                    _ => {
                        result.tag = T::Invalid;
                        self.index += 1;
                        result.range.end = self.index;
                        return result;
                    }
                },

                S::Identifier => match c {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {},
                    _ => {
                        // we ended the identifier by encountering an invalid character
                        // determine if it matches one of the keyword tags (int, void): if so set it
                        // otherwise, keep as identifier
                        let slice = &self.buffer[result.range.start .. self.index];
                        if let Some(tag) = Tag::get_keyword(slice) {
                            result.tag = tag
                        }
                        break;
                    }
                },

                S::Int => match c {
                    // TODO: add hexadecimal support here
                    // TODO: add support for floats
                    '0'..='9' => {}
                    _ => break
                }
            }

            self.index += 1;
        }

        if matches!(result.tag, T::Eof) {
            result.range.end = result.range.start;
        } else {
            result.range.end = self.index;
        }

        return result;
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod lexer_test;