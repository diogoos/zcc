use std::{path::PathBuf, process};
use clap::{arg, command, ArgAction, ArgGroup};

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

}