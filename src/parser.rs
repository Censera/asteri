use crate::lexer::Token;

pub struct parser
{
  tokens: Vec<Token>,
  current: usize,
}
