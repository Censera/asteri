mod ast;
mod lexer;
mod parser;

use std::env::*;
use std::fs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("[Use] astric <file.ast>");
        return Ok(());
    }

    let file_path = &args[1];
    let input = read_to_string(file_path)?;

    let mut lexer = lexer::Lexer::new(&input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next_token() {
        match result {
            Ok(token) => tokens.push(token),
            Err(e) => {
                eprintln!("[Error] [Lexer] {}", e);
                return Err(Box::new(e));
            }
        }
    }

    println!("\t[Source]\n{input}");
    println!("\t[Tokens]\n{:#?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse()?;

    println!("\t[Parser]\n{:#?}", stmts);
    Ok(())
}
