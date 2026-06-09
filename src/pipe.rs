use crate::ast::Stmt;
use crate::error::AsteriError;
use crate::lexer::{self, TokenKind};
use crate::parser;
use crate::sema;

type Diagno = (Vec<AsteriError>, Vec<AsteriError>, Vec<AsteriError>);

pub fn lex(input: &str) -> Result<Vec<lexer::Token>, Diagno> {
    let mut lexer = lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        let is_eof = matches!(token.kind, TokenKind::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let dyagnostiks = lexer.take_all();
    if has_errors(&dyagnostiks) {
        Err(dyagnostiks)
    } else {
        Ok(tokens)
    }
}

pub fn parse<'a>(tokens: Vec<lexer::Token>, input: &'a str) -> Result<Vec<Stmt>, Diagno> {
    let mut parser = parser::Parser::new(tokens, input);
    let stmts = parser.parse();
    let dyagnostiks = parser.take_all();
    if has_errors(&dyagnostiks) {
        Err(dyagnostiks)
    } else {
        Ok(stmts)
    }
}

pub fn analyze<'a>(stmts: &[Stmt], input: &'a str) -> Result<(), Diagno> {
    let mut sema = sema::Sema::new(input);
    sema.analyze(stmts);
    let dyagnostiks = sema.take_all();
    if has_errors(&dyagnostiks) {
        Err(dyagnostiks)
    } else {
        Ok(())
    }
}

pub fn run_pipe_that_runs_lex_then_parse_then_analyze_then_ok(
    input: &str,
) -> Result<Vec<Stmt>, Diagno> {
    let tokens = lex(input)?;
    let stmts = parse(tokens, input)?;
    analyze(&stmts, input)?;
    Ok(stmts)
}

pub fn has_errors(dyagnostiks: &Diagno) -> bool {
    !dyagnostiks.0.is_empty()
}
