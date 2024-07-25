use std::{fs, path::PathBuf, process};
use clap::{arg, command, ArgAction, ArgGroup};
mod lexer;

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

    let mut lexer = lexer::Lexer::load(&preprocessed_path);
    loop {
        let token = lexer.next();
        if matches!(token.tag, lexer::Tag::Eof) {
            println!("EOF;");
            break;
        }
        if matches!(token.tag, lexer::Tag::Invalid) {
            println!("Invalid tag!");
            process::exit(2);
        }

        let str = &lexer.buffer[token.range];
        println!("Found tag of type {:?}; value: '{}'", token.tag, str);
    }

    // Erase the preprocessed file before exiting
    if fs::remove_file(preprocessed_path).is_err() {
        println!("Failed to remove preprocessed intermediate file.");
    }
}
