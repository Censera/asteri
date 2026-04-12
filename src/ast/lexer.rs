#[derive(Debug)]
pub enum TokenKind {
  Asteri,               // *
  DoubleQuote,
  SemiComma,
  Bang,
  Let,
  Constant,
  Comma,
  Equal,
  EqualEqual,
  Colon,
  Arrow,
  True,
  False,
  If,
  Else,
  Switch,
  Function,
  While,
  Return,
  Match,
  Slash,
  Plus,
  Minus,
  RightParen,
  LeftParen,
  OpenBrace,
  CloseBrace,
  GreaterThan,
  LessThan,
  GreaterEqual,
  LessEqual,
  Public,
  Private,
  Class,
  Tilde,
  Caret,
  At,
  Dot,
  Boolean(bool),
  Character(char),
  Decimal(u32),
  Print,
  Warn,
  Error,
  Read,
  Sort,
  Str(String),
  File,
  Open,
  Close,
  Integer(i64),
  Whitespace,
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
        let number: i64 = self.swallow_number();
        kind = TokenKind::Integer(number);
      } else if c.is_whitespace() {
        self.swallow();
        kind = TokenKind::Whitespace;
      } else if c.is_alphabetic() || c == '_' {
        let id = self.swallow_id();
        kind = match id.as_str() {
          "let"     => TokenKind::Let,
          "const"   => TokenKind::Constant,
          "if"      => TokenKind::If,
          "else"    => TokenKind::Else,
          "while"   => TokenKind::While,
          "switch"  => TokenKind::Switch,
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
          "pub"     => TokenKind::Public,
          "pri"     => TokenKind::Private,
          "fun"     => TokenKind::Function,
          _         => TokenKind::Id,
        }
      } else {
        self.swallow();
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

  fn swallow_number(&mut self) -> i64 {
    let mut number: i64 = 0;
    while let Some(c) = self.current_char() {
      if c.to_digit(10).is_some() {
        self.swallow().unwrap();
        number = number * 10 + c.to_digit(10).unwrap() as i64;
      } else { 
          break;
        }
    }
    number
  }

  fn swallow_id(&mut self) -> String {
    let mut id = String::new();
    while let Some(c) = self.current_char() {
      if c.is_alphabetic() {
        self.swallow().unwrap();
        id.push(c);
      } else { break; }
    }
    id
  }
}
