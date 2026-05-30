use crate::lexer::Types;

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Neg,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    Xor,
    BitAnd,
    BitOr,
    ShiftLeft,
    ShiftRight,
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
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
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
    Ret(Option<Expr>),
}
