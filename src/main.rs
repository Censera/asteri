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
    let lines: Vec<&str> = input.lines().collect();

    let mut lexer = lexer::Lexer::new(&input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let (errors, warnings, info) = lexer.take_all();

    if error::report_and_check(errors, warnings, info) {
        return Ok(());
    }

    // println!("\t<Tokens>\n{:#?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse()?;

    // println!("\t<Parser>\n{:#?}", stmts);

    // let mut interpreter = interpreter::Interpreter::new();
    // interpreter.run(&stmts)?;

    let mut sema = sema::Sema::new();
    sema.analyze(&stmts)?;

    Ok(())
}
