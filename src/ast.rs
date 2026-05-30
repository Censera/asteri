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
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum MatchPattern {
    Expr(Expr),
    Defualt,
}

#[derive(Debug)]
pub enum Expr {
    Int(isize),
    Bool(bool),
    Str(String),
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
    Error(Expr),

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

    While {
        condtion: Expr,
        body: Vec<Stmt>,
    },

    Loop {
        body: Vec<Stmt>,
    },

    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },

    Block(Vec<Stmt>),
    Ret(Option<Expr>),
}
