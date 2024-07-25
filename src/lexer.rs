use std::{fs, ops::RangeBounds, path::PathBuf, process};
use regex::Regex;
use lazy_static::lazy_static;

enum LexicalToken {
    Identifier(String),
    Constant(String),
    
    Int, // int keyword
    Void, // void keyword
    Ret, // return keyword

    Po, // open parenthesis `(`
    Pc, // close parenthesis `)`
    Bo, // open brace `{`
    Bc, // close brace `}`
    Semicolon
}

lazy_static! {
    static ref KEYWORD_RX: Regex = Regex::new("[a-zA-Z_]\\w*\\b").unwrap();
    static ref CONSTANT_RX: Regex = Regex::new("[0-9]+\\b").unwrap();
    
    static ref INT_RX: Regex = Regex::new("int\\b").unwrap();
    static ref VOID_RX: Regex = Regex::new("void\\b").unwrap();
    static ref RET_RX: Regex = Regex::new("return\\b").unwrap();

    static ref P_OPEN_RX: Regex = Regex::new("\\(").unwrap();
    static ref P_CLOSE_RX: Regex = Regex::new("\\)").unwrap();
    static ref B_OPEN_RX: Regex = Regex::new("\\{").unwrap();
    static ref B_CLOSE_RX: Regex = Regex::new("\\}").unwrap();
    
    static ref SEMICOLON_RX: Regex = Regex::new(";").unwrap();
}

pub fn lex(file: &PathBuf) {
    let mut content = fs::read_to_string(file).expect("Error: unable to read preprocessed file");
    let mut tokens: Vec::<LexicalToken> = Vec::default();

    while !content.is_empty() {
        content = content.trim_start().to_string();

        // Match on keywords
        if let Some(m) = KEYWORD_RX.find(&content) {
            let t = m.as_str();

            // Check for int, void, ret
            if INT_RX.is_match(t) {
                println!("Found `int`");
                tokens.push(LexicalToken::Int);
                content.replace_range(m.range(), "");
                continue;
            }

            if VOID_RX.is_match(t) {
                println!("Found `void`");
                tokens.push(LexicalToken::Void);
                content.replace_range(m.range(), "");
                continue;
            }

            if RET_RX.is_match(t) {
                println!("Found `return`");
                tokens.push(LexicalToken::Ret);
                content.replace_range(m.range(), "");
                continue;
            }

            // Otherwise, treat as identifier
            let id = t.to_string();
            println!("Found identifier `{}`", id);
            tokens.push(LexicalToken::Identifier(id));
            content.replace_range(m.range(), "");
        }
        // Match on constants
        else if let Some(m) = CONSTANT_RX.find(&content) {
            println!("Found constant `{}`", m.as_str());
            println!("~ Constant context: [{}]", content);


            tokens.push(LexicalToken::Constant(m.as_str().to_string()));
            content.replace_range(m.range(), "");
        }
        // Match on brackets
        else if let Some(m) = P_OPEN_RX.find(&content) {
            println!("Found P_OPEN");
            tokens.push(LexicalToken::Po);
            content.replace_range(m.range(), "");
        }
        else if let Some(m) = P_CLOSE_RX.find(&content) {
            println!("Found P_CLOSE");
            tokens.push(LexicalToken::Pc);
            content.replace_range(m.range(), "");
        }
        else if let Some(m) = B_OPEN_RX.find(&content) {
            println!("Found B_OPEN");
            tokens.push(LexicalToken::Bo);
            content.replace_range(m.range(), "");
        }
        else if let Some(m) = B_CLOSE_RX.find(&content) {
            println!("Found B_CLOSE");
            tokens.push(LexicalToken::Bc);
            content.replace_range(m.range(), "");
        }
        else if let Some(m) = SEMICOLON_RX.find(&content) {
            println!("Found Semicolon");
            tokens.push(LexicalToken::Semicolon);
            content.replace_range(m.range(), "");
        } else if content.len() != 0 {
            println!("LEXER ERROR!");
            process::exit(2);
        }
    }
}