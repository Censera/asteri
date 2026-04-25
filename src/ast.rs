use crate::lexer::{ TokenKind };

pub enum Expr
{
  Int(isize),
  Float(f64),
  Str(String),
  Char(char),
  Bool(bool),
  Id(String),
  BinOp { li: Box<Expr>, op: TokenKind, ri: Box<Expr> },
  Unary { op: TokenKind, expr: Box<Expr> },
}

pub enum Stmt
{
  Let { name: String, value: Expr },
  Const { name: String, value: Expr },
  Return(Expr),
  Expr(Expr),
}
