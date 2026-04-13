mod ast;

fn main() {
  let input = r#"-> "Hello, world!"        &&a hola || | == <= >= !="ada"#;

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
      tokens.push(token);
    }
   println!("{}\n{:#?}", input, tokens);
}
