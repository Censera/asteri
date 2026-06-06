pub use crate::lexer::Types;

#[derive(Debug)]
pub enum UnaryOp {
    Not,    // !
    Minus,  // -
    BitNot, // ~
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
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

    BitOr,  // bor
    BitAnd, // band
    BitXor, // xor
}

#[derive(Debug)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum MatchPattern {
    Expr(Expr),
    Default,
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub tp: Types,
}

#[derive(Debug)]
pub struct StructMethod {
    pub is_immut: bool,
    pub rt_tp: Option<Types>,
    pub name: String,
    pub params: Vec<(String, Types)>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Float(f64),
    Str(String),
    Unit, // ()

    Id {
        name: String,
        line: usize,
    },

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        line: usize,
    },
    Call {
        name: String,
        args: Vec<Expr>,
        line: usize,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        line: usize,
    },

    Reference(Box<Expr>), // &x
    Dereference {
        expr: Box<Expr>,
        line: usize,
    }, // p^
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Error(Expr),

    Let {
        name: String,
        tp: Option<Types>,
        value: Option<Expr>,
        line: usize,
    },

    Immut {
        name: String,
        tp: Option<Types>,
        value: Option<Expr>,
        line: usize,
    },

    Assign {
        name: String,
        value: Expr,
        line: usize,
    },

    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_branch: Option<Box<Stmt>>,
        line: usize,
    },

    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },

    Loop {
        body: Vec<Stmt>,
    },

    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },

    Fun {
        rt_tp: Option<Types>,
        name: String,
        params: Vec<(String, Types)>,
        body: Vec<Stmt>,
        line: usize,
    },

    Call {
        name: String,
        args: Vec<Expr>,
        line: usize,
    },

    Struct {
        name: String,
        fields: Vec<StructField>,
        methods: Vec<StructMethod>,
    },

    Ret {
        expr: Option<Expr>,
        line: usize,
    },

    CBlock(String),
    Block(Vec<Stmt>),
}
