#[derive(Debug)]
pub enum TokenKind {
  // Literals
  Int(isize),
  // Operators
  Asteri,
  Dot,
  Comma,
  Colon,
  Semicolon,
  Bang,
  Huh,
  At,
  Caret,
  Ampersand,
  Minus,
  Underscore,
  Plus,
  Equals,
  Slash,
  Pipe,
  Tilde,
  SingleQuote,
  LessThan,
  GreaterThan,
  OpeningBrace,
  ClosingBrace,
  OpeningParen,
  ClosingParen,
  OpeningBracket,
  ClosingBracket,

  // Keywords
  Let,
  Constant,
  If,
  Else,
  While,
  Match,
  Break,
  Return,
  Continure,
  True,
  False,
  Null,
  New,
  Class,
  Function,
  Pub,
  Pri,
  Enum,
  Struct,
  Using,
  Print,
  Read,
  Warn,
  Error,
  Open,
  Close,
  Sort,

  // Other
  Unknown,
  EOF,
  Id,
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
  current: usize,
}

impl <'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input, 
      current: 0,
    }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    while matches!(self.current_char(), Some(c) if c.is_whitespace()) {
      self.swallow();
    }

    if self.current > self.input.len() {
      return None;
    }
    if self.current == self.input.len() {
      self.current += 1;
      return Some(Token::new(
          TokenKind::EOF,
          TextSpan::new(0, 0, '\0'.to_string())
      ));
    }

  let c = self.current_char();
    return c.map(|c| {
      let start = self.current;
      let mut kind = TokenKind::Unknown;
    if c.is_digit(10) {
        let number: isize = self.swallow_number();
        kind = TokenKind::Int(number);
      } else if c.is_whitespace() {
        while let Some(c) = self.current_char() {
          if c.is_whitespace() { self.swallow(); } else { break; }
        }
      } else if c.is_alphabetic() || c == '_' || c.is_digit(10) {
        let id = self.swallow_id();
        kind = match id.as_str() {
          "let"     => TokenKind::Let,
          "const"   => TokenKind::Constant,
          "if"      => TokenKind::If,
          "else"    => TokenKind::Else,
          "while"   => TokenKind::While,
          "match"   => TokenKind::Match,
          "print"   => TokenKind::Print,
          "read"    => TokenKind::Read,
          "warn"    => TokenKind::Warn,
          "error"   => TokenKind::Error,
          "open"    => TokenKind::Open,
          "close"   => TokenKind::Close,
          "sort"    => TokenKind::Sort,
          "true"    => TokenKind::True,
          "false"   => TokenKind::False,
          "class"   => TokenKind::Class,
          "return"  => TokenKind::Return,
          "pub"     => TokenKind::Pub,
          "pri"     => TokenKind::Pri,
          "fun"     => TokenKind::Function,
          _         => TokenKind::Id,
        }
      } else {
        self.swallow();
        kind = match c {
          '+'      => TokenKind::Plus,
          '-'      => TokenKind::Minus,
          '*'      => TokenKind::Asteri,
          '/'      => TokenKind::Slash,
          ','      => TokenKind::Comma,
          ':'      => TokenKind::Colon,
          ';'      => TokenKind::Semicolon,
          '@'      => TokenKind::At,
          '^'      => TokenKind::Caret,
          '='      => TokenKind::Equals,
          '|'      => TokenKind::Pipe,
          '~'      => TokenKind::Tilde,
          '<'      => TokenKind::LessThan,
          '>'      => TokenKind::GreaterThan,
          '('      => TokenKind::OpeningParen,
          ')'      => TokenKind::ClosingParen,
          '{'      => TokenKind::OpeningBrace,
          '}'      => TokenKind::ClosingBrace,
          '['      => TokenKind::OpeningBracket,
          ']'      => TokenKind::ClosingBracket,
          _        => TokenKind::Unknown,
        }
      }
    
      let end: usize = self.current;
      let literal: String = self.input[start..end].to_string();
      let span  =  TextSpan::new(start, end, literal);
      Token::new(kind, span)
    });
  }

  fn current_char(&self) -> Option<char> {
    self.input.chars().nth(self.current)
  }
  fn swallow(&mut self) -> Option<char> {
    if self.current >= self.input.len() {
      return None;
    }
    let c = self.current_char();
    self.current += 1;
    
    c
  }

  fn swallow_number(&mut self) -> isize {
    let mut number: isize = 0;
    while let Some(c) = self.current_char() {
      if c.to_digit(10).is_some() {
        self.swallow().unwrap();
        number = number * 10 + c.to_digit(10).unwrap() as isize;
      } else { 
          break;
        }
    }
    number
  }

  fn swallow_id(&mut self) -> String {
    let mut id = String::new();
    while let Some(c) = self.current_char() {
      if c.is_alphabetic() || c == '_' || c.is_digit(10) {
        self.swallow().unwrap();
        id.push(c);
      } else { break; }
    }
    id
  }
}
