mod lexer;

fn main()
{
	let input = r#"form feht4gt new pri enum boo {} pub struct foo { age: int } let grade: char = 'A'; let myInput: string = "Your input number is: ", read -> int;
                i32, float 16.45"#;

	let mut lexer = lexer::Lexer::new(input);
	let mut tokens = Vec::new();
	while let Some(result) = lexer.next_token()
	{
		match result
		{
			Ok(token) => tokens.push(token),
			Err(e) =>
			{
				eprintln!("lex error:\n{}", e);
				break;
			}
		}
	}
	println!("{}\n{:#?}", input, tokens);
}
