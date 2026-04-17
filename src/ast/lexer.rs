#[derive(Debug)]
pub enum TokenKind {
  // Literals
  Int(isize),
  Str(String),
  Char(char),
  Float(f64),
  // Types
  Type(Types),

  // Operators
  Asteri,
  Dot,
  Comma,
  Colon,
  Semicolon,
  Huh,
  At,
  Caret,
  Ampersand,
  Minus,
  Underscore,
  Plus,
  Equal,
  Slash,
  Pipe,
  Tilde,
  LessThan,
  GreaterThan,
  OpeningBrace,
  ClosingBrace,
  OpeningParen,
  ClosingParen,
  OpeningBracket,
  ClosingBracket,

  // Compound Char
  Arrow,
  NotEqual,
  EqualEqual,
  LessOrEqual,
  GreaterOrEqual,
  TwoPipes,
  TwoAmpersands,

  // Keywords
  Let,
  Const,
  If,
  Else,
  While,
  Match,
  Break,
  Return,
  Continue,
  True,
  False,
  None,
  New,
  Form,
  Fun,
  Pub,
  Pri,
  Enum,
  Struct,
  Using,
  Print,
  Write,
  Read,
  Append,
  Warn,
  Error,
  Open,
  Close,
  Sort,

  // Other
  Unknown,
  EOF,
  Id(String),
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
pub enum Types {
  I8, I16, I32, I64,
  U8, U16, U32, U64,
  F32, F64,
  String, Boolean, Char, File
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
  chars: Vec<char>,
  current: usize,
}

impl <'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input, 
      chars: input.chars().collect(),
      current: 0,
    }
  }

  pub fn next_token(&mut self) -> Option<Token> {
    while matches!(self.current_char(), Some(c) if c.is_whitespace()) {
      self.swallow();
    }

    if self.current > self.chars.len() {
      return None;
    }
    if self.current == self.chars.len() {
      self.current += 1;
      return Some(Token::new(
          TokenKind::EOF,
          TextSpan::new(0, 0, '\0'.to_string())
      ));
    }

  let c = self.current_char();
    return c.map(|c| {
      let start = self.current;
      let mut kind: TokenKind;
    if c.is_digit(10) {
        kind = self.consume_number();
      } else if c.is_alphabetic() || c == '_' || c.is_digit(10) {
        let id = self.consume_id();
        kind = match id.as_str() {
          "let"     => TokenKind::Let,
          "const"   => TokenKind::Const,
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
          "form"    => TokenKind::Form,
          "new"     => TokenKind::New,
          "return"  => TokenKind::Return,
          "break"   => TokenKind::Break,
          "continue"=> TokenKind::Continue, 
          "using"   => TokenKind::Using,
          "pub"     => TokenKind::Pub,
          "enum"    => TokenKind::Enum,
          "struct"  => TokenKind::Struct,
          "pri"     => TokenKind::Pri,
          "fun"     => TokenKind::Fun,
          "none"    => TokenKind::None,
          "file"    => TokenKind::Type(Types::File),
          "int"     => TokenKind::Type(Types::I32),
          "i8"      => TokenKind::Type(Types::I8),
          "i16"     => TokenKind::Type(Types::I16),
          "i32"     => TokenKind::Type(Types::I32),
          "i64"     => TokenKind::Type(Types::I64),
          "u8"      => TokenKind::Type(Types::U8),
          "u16"     => TokenKind::Type(Types::U16),
          "u32"     => TokenKind::Type(Types::U32),
          "u64"     => TokenKind::Type(Types::U64),
          "float"   => TokenKind::Type(Types::F32),
          "f32"     => TokenKind::Type(Types::F32),
          "f64"     => TokenKind::Type(Types::F64),
          "bool"    => TokenKind::Type(Types::Boolean),
          "string"  => TokenKind::Type(Types::String),
          "char"    => TokenKind::Type(Types::Char),
          _         => TokenKind::Id(id),
        }
      } else {
        self.swallow();
        kind = match c {
          '+'      => TokenKind::Plus,
          '-'      => self.is_compound('>', TokenKind::Arrow, TokenKind::Minus),
          '*'      => TokenKind::Asteri,
          '/'      => TokenKind::Slash,
          ','      => TokenKind::Comma,
          '.'      => TokenKind::Dot,
          ':'      => TokenKind::Colon,
          ';'      => TokenKind::Semicolon,
          '@'      => TokenKind::At,
          '^'      => TokenKind::Caret,
          '?'      => TokenKind::Huh,
          '='      => self.is_compound('=', TokenKind::EqualEqual, TokenKind::Equal),
          '!'      => self.is_compound('=', TokenKind::NotEqual, TokenKind::Unknown),
          '|'      => self.is_compound('|', TokenKind::TwoPipes, TokenKind::Pipe),
          '&'      => self.is_compound('&', TokenKind::TwoAmpersands, TokenKind::Ampersand),
          '~'      => TokenKind::Tilde,
          '<'      => self.is_compound('=', TokenKind::LessOrEqual, TokenKind::LessThan),
          '>'      => self.is_compound('=', TokenKind::GreaterOrEqual, TokenKind::GreaterThan),
          '_'      => TokenKind::Underscore,
          '('      => TokenKind::OpeningParen,
          ')'      => TokenKind::ClosingParen,
          '{'      => TokenKind::OpeningBrace,
          '}'      => TokenKind::ClosingBrace,
          '['      => TokenKind::OpeningBracket,
          ']'      => TokenKind::ClosingBracket,
          '"'      => self.string_token(),
          '\''     => self.char_token(),
          _        => TokenKind::Unknown,
        }
      }
    
      let end: usize = self.current;
      let literal: String = self.chars[start..end].iter().collect();
      let span  =  TextSpan::new(start, end, literal);
      Token::new(kind, span)
    });
  }

  fn current_char(&self) -> Option<char> {
    self.chars.get(self.current).copied()
  }
  fn swallow(&mut self) -> Option<char> {
    if self.current >= self.chars.len() {
      return None;
    }
    let c = self.chars[self.current];
    self.current += 1;
    
    Some(c)
  }

  fn consume_number(&mut self) -> TokenKind
  {
    let mut int_part: isize = 0;
    while let Some(c) = self.current_char()
    {
      if c.is_digit(10)
      {
        self.swallow().unwrap();
        int_part = int_part * 10 + (c as isize - '0' as isize);
      }
      else
      {
        break;
      }
    }
    if self.current_char() == Some('.')
    {
      let next_char = self.chars.get(self.current +1).copied();
      if next_char.map_or(false, |c| c.is_digit(10))
      {
        self.swallow();
        let mut fraction: f64 = 0.0;
        let mut place = 0.1;
        while let Some(c) = self.current_char()
        {
          if c.is_digit(10)
          {
            self.swallow();
            fraction += (c as u8 - b'0') as f64 * place;
            place *= 0.1;
          }
          else 
          {
            break;
          }
        }
          return TokenKind::Float(int_part as f64 + fraction);
      }
    }
      TokenKind::Int(int_part)
  }

  fn consume_id(&mut self) -> String {
    let mut id = String::new();
    while let Some(c) = self.current_char() {
      if c.is_alphabetic() || c == '_' || c.is_digit(10) {
        self.swallow().unwrap();
        id.push(c);
      } else { break; }
    }
    id
  }

  fn is_compound( &mut self,
                  expected: char,
                  compound_operator: TokenKind,
                  operator: TokenKind )
                  -> TokenKind {
    if let Some(next) = self.current_char() {
      if next == expected {
        self.swallow();
        compound_operator
      } else { operator }
    }   else { operator }
  }

  fn string_token(&mut self) -> TokenKind {
    let mut content = String::new();
    while (self.current_char() != Some('"')) && !self.is_eof() {
      content.push(self.swallow().unwrap());
    }
    if self.current_char() != Some('"') {
      panic!("expected closing double quotes");
    }

    self.swallow();
    TokenKind::Str(content)
  } 

  fn char_token(&mut self) -> TokenKind {
    let c = match self.current_char() {
      Some('\\') => {
        self.swallow();
        match self.current_char() {
          Some('n')   => { self.swallow(); '\n' }
          Some('t')   => { self.swallow(); '\t' }
          Some('r')   => { self.swallow(); '\r' }
          Some('\'')  => { self.swallow(); '\'' }
          Some('\\')  => { self.swallow(); '\\' }
          other       => panic!("unexpected character, {:?}", other),
        }
      }
      Some(ch) if !ch.is_whitespace() => { self.swallow(); ch }
      _ => panic!("invalid character"),
    };

    if self.current_char() != Some('\'') {
      panic!("expected closing quote");
    }
    self.swallow();
    
    TokenKind::Char(c)
  }

  fn is_eof(&self) -> bool{
    self.current >= self.chars.len()
  }
}
