use crate::lexer::Types;

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Neg,
    Mns,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    BitOr,
    BitAnd,
    BitXor,
    Eql,
    Neq,
    LessTh,
    GreaTh,
    LessOr,
    GreaOr,
    LogicOr,
    LogicAnd,
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
    Erro(Expr),

    Let {
        name: String,
        tp: Types,
        value: Expr,
    },

    Const {
        name: String,
        tp: Types,
        value: Expr,
    },

    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Block(Vec<Stmt>),

    Ret(Option<Expr>),
}
