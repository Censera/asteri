use crate::lexer::Token;
use crate::ast::Expr;

pub struct parser
{
  tokens: Vec<Token>,
  current: usize,
}
