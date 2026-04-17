mod ast;

fn main() {
  let input = r#"form new pri enum boo {} pub struct foo { age: int } let grade: char = 'A'; let myInput: string = "Your input number is: ", read -> int;
                i32, float 16.45"#;

    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
      tokens.push(token);
    }
   println!("{}\n{:#?}", input, tokens);
}
