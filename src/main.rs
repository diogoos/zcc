use core::panic;
use std::{fs, io::Write, path::PathBuf, process};
use clap::{arg, command, ArgAction, ArgGroup};
mod debug;
use debug::dprintln;

mod lex;
use lex::lexer;
mod ast;
use ast::parser;
mod zil;
mod assembly;

fn main() {
    let matches = command!()
        .arg(arg!([path] "Path to operate on")
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf))
                )
        .arg(arg!(lex: --lex "Runs the lexer, but stops before parsing").action(ArgAction::SetTrue))
        .arg(arg!(parse: --parse "Runs the lexer and parser, but stops before assembly generation").action(ArgAction::SetTrue))
        .arg(arg!(tacky: --tacky "Runs the compiler through the ZIL generation stage, stopping before assembly generation").action(ArgAction::SetTrue))
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


    // - 1. Run the lexer
    let mut lexer = lexer::Lexer::load(&preprocessed_path);
    let mut tokens = vec![];
    loop {
        let token = lexer.next();
        if matches!(token.tag, lexer::Tag::Eof) {
            dprintln!("Found EOF;");
            break;
        }
        if matches!(token.tag, lexer::Tag::Invalid) {
            let str = &lexer.buffer[token.range.clone()];
            println!("Lexer error: encountered invalid tag `{}` in range {}-{}", str, token.range.start, token.range.end);
            process::exit(2);
        }

        dprintln!("Found tag of type {:?}; value: '{}'", token.tag, &lexer.buffer[token.range.clone()]);
        tokens.push(token);
    }
    dprintln!("Lexed file successfully.\n");

    // Erase the preprocessed file, as it is no longer necessary
    if fs::remove_file(preprocessed_path).is_err() {
        println!("Failed to remove preprocessed intermediate file.");
    }
    // If we are just lexing, exit gracefully if succeeded
    if matches.get_flag("lex") {
        process::exit(0);
    }


    // - 2. Parse the tokens
    let mut t = parser::ASTParser::new(lexer.buffer, tokens);

    let result = t.parse();
    let ast_tree: ast::symbols::Program;
    match result {
        Ok(program_tree) => {
            dprintln!("Built AST successfully.");
            dprintln!("{:#?}", &program_tree);

            if matches.get_flag("parse") {
                process::exit(0);
            }
            
            ast_tree = program_tree;
        },
        Err(e) => {
            panic!("{}", e);
        }
    }

    // - 3. Convert the Tree to Z intermediate language
    let intermediate: zil::symbols::Program = zil::transpile::parse(ast_tree.clone());
    dprintln!("\nTranspiled to ZIL successfully.");
    dprintln!("{:#?}", intermediate);
    if matches.get_flag("tacky") {
        process::exit(0);
    }

    // - 4. Compile ZIL into Assembly
    let mut transpiler = assembly::transpile::STranspiler::new();
    let assembled = transpiler.parse(intermediate);
    dprintln!("\nTranspiled to Assembly successfully.");
    dprintln!("{:#?}", assembled);


    // - 5. Codegen from assembly output
    let code = assembly::agen::codegen(&assembled);
    let should_output = matches.get_flag("assemble");
    dprintln!("\nCodegen successful!");
    dprintln!("{}", code);

    // - 6. Assemble and link
    if matches.get_flag("codegen") && !should_output {
        process::exit(0);
    }

    let assembly_path = path.clone().with_extension("s");
    let output_path = path.clone().with_extension("");

    let mut file_handle = fs::File::create(&assembly_path).expect("IOError: Unable to create output file");
    write!(file_handle, "{}", code).expect("IOError: Unable to write to output file");

    if matches.get_flag("codegen") {
        process::exit(0);
    }

    let mut assemble = process::Command::new("gcc");
    assemble.arg(assembly_path.clone().into_os_string())
              .arg("-o")
              .arg(&output_path.into_os_string());
    assemble.status().expect("GCC Error: failed to assemble the output!");
    drop(assemble);

    if !should_output {
        fs::remove_file(&assembly_path).expect("IOError: Unable to delete assembly intermediate");
    }
}
