pub use crate::lexer::Types;

#[derive(Debug)]
#[allow(dead_code)]
pub enum UnaryOp {
    Not,    // !
    Minus,  // -
    BitNot, // ~
}

#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MatchPattern {
    Expr(Expr),
    Default,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StructField {
    pub name: String,
    pub tp: Types,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StructMethod {
    pub is_immut: bool,
    pub rt_tp: Option<Types>,
    pub name: String,
    pub params: Vec<(String, Types)>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug)]
#[allow(dead_code)]
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
        depth: usize,
        line: usize,
    }, // p^

    Lambda {
        params: Vec<String>,
        body: LambdaBody,
        line: usize,
    },

    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        line: usize,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Stmt {
    Expr {
        expr: Expr,
        line: usize,
    },
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

    Assign {
        target: Expr,
        value: Expr,
        line: usize,
    },

    Break {
        line: usize,
    },

    Continue {
        line: usize,
    },

    CBlock(String),
    Block(Vec<Stmt>),

    Thunk {
        name: String,
        body: LambdaBody,
        line: usize,
    },
}
