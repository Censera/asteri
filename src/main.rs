mod lexer;

fn main()
{
	let input = r#"print "Hello, World!", !true, 43;"#;

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
