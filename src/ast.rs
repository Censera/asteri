pub use crate::lexer::Types;

#[derive(Debug)]
#[allow(dead_code)]
pub enum UnaryOp {
    // !
    Not,
    // -
    Minus,
    // >~
    BitNot,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinaryOp {
    // +
    Add,
    // -
    Sub,
    // *
    Mul,
    // /
    Div,
    // =
    Eql,
    // !=
    Neq,
    // <
    LessTh,
    // >
    GreaTh,
    // <=
    LessOr,
    // >=
    GreaOr,
    // ||
    LogicOr,
    // &&
    LogicAnd,
    // <<
    ShiftLeft,
    // >>
    ShiftRight,
    // >|
    BitOr,
    // >&
    BitAnd,
    // >@
    BitXor,
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
    pub vis: Vis,
    pub name: String,
    pub tp: Types,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StructMethod {
    pub vis: Vis,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vis {
    Pub,
    Pri,
    Inh,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expr {
    Non,

    Int(i64),
    Bool(bool),
    Float(f64),
    Char(char),
    Str(String),
    Unit, // ()

    Cast {
        expr: Box<Expr>,
        target: Types,
        line: usize,
    },

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

    // &x
    Reference(Box<Expr>),

    // p^
    Dereference {
        expr: Box<Expr>,
        depth: usize,
        line: usize,
    },

    Lambda {
        params: Vec<String>,
        body: LambdaBody,
        line: usize,
    },

    StructLit {
        name: String,
        fields: Vec<(String, Expr)>,
        line: usize,
    },

    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        line: usize,
    },

    FmtStr {
        raw: String,
        line: usize,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Stmt {
    Non,

    Expr {
        expr: Expr,
        line: usize,
    },
    Print(Expr),
    Error(Expr),

    Let {
        vis: Vis,
        name: String,
        tp: Option<Types>,
        value: Option<Expr>,
        line: usize,
    },

    Immut {
        vis: Vis,
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
        vis: Vis,
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
        vis: Vis,
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
