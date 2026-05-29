use crate::lexer::Types;

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LtOrEqual,
    GtOrEqual,
}

#[derive(Debug)]
pub enum Expr {
    Int(isize),
    Bool(bool),
    Id(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Let {
        name: String,
        tp: Types,
        value: Expr,
    },
    Ret(Expr),
}
