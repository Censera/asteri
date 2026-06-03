mod ast;
mod error;
mod lexer;
mod parser;
mod sema;

use std::env::*;
use std::fs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("\t<Use>\nac [file.ast]");
        return Ok(());
    }

    let file_path = &args[1];
    let input = read_to_string(file_path)?;

    // Lexer
    let mut lexer = lexer::Lexer::new(&input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    let (errors, warnings, info) = lexer.take_all();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    // Parser
    let mut parser = parser::Parser::new(tokens, &input);
    let stmts = parser.parse();
    let (errors, warnings, info) = parser.take_all();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    // Sematic Analysis
    let mut sema = sema::Sema::new(&input);
    sema.analyze(&stmts);
    let (errors, warnings, info) = sema.take_all();
    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    Ok(())
}
