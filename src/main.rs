mod lexer;
mod ast;
mod parser;

use std::env::*;
use std::fs::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{

  let args: Vec<String> = args().collect();
  
  if args.len() < 2
  {
    eprintln!("Use: ast <file.ast>");
    return Ok(())
  }

  let file_path = &args[1];
  let input = read_to_string(file_path)?;

	let mut lexer = lexer::Lexer::new(&input);
	let mut tokens = Vec::new();

	while let Some(result) = lexer.next_token()
	{
		match result
		{
			Ok(token) => tokens.push(token),
			Err(e) =>
			{
				eprintln!("lex error: {}", e);
				return Err(Box::new(e))
			}
		}
	}
  println!("\nSource:\n{}\n\nTokens:\n{:#?}", input, tokens);
  Ok(())
}
