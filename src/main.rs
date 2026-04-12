mod ast;

fn main() {
    let input = r#"print sort   warn 4 class fun"#;

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
      tokens.push(token);
    }
   println!("{}\n{:#?}", input, tokens);
}
