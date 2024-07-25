use std::{fs, path::PathBuf, process};
use clap::{arg, command, ArgAction, ArgGroup};
mod lexer;
mod ast_definitions;
mod ast;

fn main() {
    let matches = command!()
        .arg(arg!([path] "Path to operate on")
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf))
                )
        .arg(arg!(lex: --lex "Runs the lexer, but stops before parsing").action(ArgAction::SetTrue))
        .arg(arg!(parse: --parse "Runs the lexer and parser, but stops before assembly generation").action(ArgAction::SetTrue))
        .arg(arg!(codegen: --codegen "Runs the lexer, parser and assembly generation, but stops before code emission").action(ArgAction::SetTrue))
        .arg(arg!(assemble: -S --assemble "Emits an assembly file (if generated), but does not link it").action(ArgAction::SetTrue))
        .arg(arg!(verbose: -v --verbose "Verbose output"))
        .group(ArgGroup::new("directives")
                            .args(["lex", "parse", "codegen"])
                            .multiple(false)
                            .required(false)
        )
        .get_matches();

    let path: &PathBuf = matches.get_one("path").expect("Path to operate on is required!");
    if !path.exists() {
        eprintln!("Error: invalid input file path provided");
        process::exit(128);
    }

    // Preprocess the files using GCC (as zcc only acts as a compiler)
    let preprocessed_path = path.clone().with_extension("i"); // output to same file with `.i` extension
    let mut preprocess = process::Command::new("gcc");
    preprocess.arg("-E") // run only the preprocessor
              .arg("-P") // don't emit linemarkers
              .arg(path.clone().into_os_string())
              .arg("-o")
              .arg(preprocessed_path.clone().into_os_string());
    preprocess.status().expect("GCC Error: failed to preprocess the given input!");
    drop(preprocess);


    // - 1. Run the lexer
    let mut lexer = lexer::Lexer::load(&preprocessed_path);
    let mut tokens = vec![];
    loop {
        let token = lexer.next();
        if matches!(token.tag, lexer::Tag::Eof) {
            if matches.get_flag("verbose") { println!("Found EOF;") }
            break;
        }
        if matches!(token.tag, lexer::Tag::Invalid) {
            let str = &lexer.buffer[token.range.clone()];
            println!("Lexer error: encountered invalid tag `{}` in range {}-{}", str, token.range.start, token.range.end);
            process::exit(2);
        }

        if matches.get_flag("verbose") {
            let str = &lexer.buffer[token.range.clone()];
            println!("Found tag of type {:?}; value: '{}'", token.tag, str);
        }
        tokens.push(token);
    }
    if matches.get_flag("verbose") { println!("Lexed file successfully.") }

    // Erase the preprocessed file, as it is no longer necessary
    if fs::remove_file(preprocessed_path).is_err() {
        println!("Failed to remove preprocessed intermediate file.");
    }
    // If we are just lexing, exit gracefully if succeeded
    if matches.get_flag("lex") {
        process::exit(0);
    }


    // - 2. Parse the tokens
    let mut t = ast::ASTParser::new(lexer.buffer, tokens);
    let result = t.parse();
    match result {
        Ok(program_vector) => {
            println!("{:#?}", program_vector);
        },
        Err(e) => {
            match e {
                ast::ASTError::SyntaxError(msg) => {
                    println!("Syntax error: {}", msg);
                    process::exit(6);
                },
                ast::ASTError::InternalParserError(msg) => {
                    println!("Internal parser error: {}", msg);
                    process::exit(9);
                }
            }            
        }
    }
}
