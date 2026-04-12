#[derive(Debug)]
pub enum TokenKind {
  Astri,                // *
  Slash,                // /
  Plus,                 // +
  Minus,                // -
  RightParen,           // )
  LeftParen,            // (
  Number(i64),          // 0..9
  Unknown,              // _
  EOF,                  // End of File '\0'
}

#[derive(Debug)]
pub struct TextSpan {
  start:  usize,
  end:    usize,
  literal:  String,
}

impl TextSpan {
  pub fn new(start: usize, end: usize, literal: String) -> Self {
    Self {
      start,
      end,
      literal
    }
  }

  pub fn length(&self) -> usize {
    self.end - self.start
  }
}

#[derive(Debug)]
pub struct Token {
  kind: TokenKind,
  span: TextSpan,
}

impl Token {
  pub fn new(kind: TokenKind, span: TextSpan) -> Self {
    Self {
      kind,
      span
    }
  }
}

pub struct Lexer<'a> {
  input: &'a str,
  current_pos: usize,
}

impl <'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input, 
      current_pos: 0,
    }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    if self.current_pos > self.input.len() {
      return None;
    }
    if self.current_pos == self.input.len() {
      self.current_pos += 1;
      return Some(Token::new(
          TokenKind::EOF,
          TextSpan::new(0, 0, '\0'.to_string())
      ));
    }

  let c = self.current_char();
    return c.map(|c| {
      let start = self.current_pos;
      let mut kind = TokenKind::Unknown;
      if Self::is_number_start(&c) {
        let number: i64 = self.consume_number();
        kind = TokenKind::Number(number);
      } else {
      self.consume();
      }
    
      let end: usize = self.current_pos;
      let literal: String = self.input[start..end].to_string();
      let span  =  TextSpan::new(start, end, literal);
      Token::new(kind, span)
    });
  }
  fn is_number_start(c: &char) -> bool {
    c.is_digit(10)
  }
  fn current_char(&self) -> Option<char> {
    self.input.chars().nth(self.current_pos)
  }
  fn consume(&mut self) -> Option<char> {
    if self.current_pos >= self.input.len() {
      return None;
    }
    let c = self.current_char();
    self.current_pos += 1;
    
    c
  }

  fn consume_number(&mut self) -> i64 {
    let mut number: i64 = 0;
    while let Some(c) = self.current_char() {
      if c.to_digit(10).is_some() {
        self.consume().unwrap();
        number = number * 10 + c.to_digit(10).unwrap() as i64;
      } else { 
          break;
        }
      }
    number
  }
}
