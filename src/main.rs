mod ast;

fn main() {
    let input = "4";

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
      tokens.push(token);
    }
   println!("{:?}", tokens);
}
