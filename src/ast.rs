use crate::lexer::{ Types }  ;

pub enum BinaryOp {
    Add,
    Sub,
    Mu,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LtOrEqual,
    GtOrEqual,
}

pub enum Expr {
    Int(isize),
    Bool(bool),
    Id(String),
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

pub enum Stmt {
    Print(Expr),
    Let { name: String, tp: Types, value: Expr },
    Return(Expr),
}
