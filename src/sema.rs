use crate::ast::{BinaryOp, Expr, LambdaBody, Stmt, Types, UnaryOp};
use crate::error::{AsteriError, ErrorKind};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Symbol {
    Variable {
        tp: Types,
        immut: bool,
    },

    Fun {
        params: Vec<Types>,
        return_type: Box<Types>,
    },

    Struct(String),
}

#[derive(Debug, Clone)]
struct StructDef {
    fields: HashMap<String, Types>,
    methods: HashMap<String, Symbol>,
}

pub struct Sema<'a> {
    input: &'a str,
    scopes: Vec<HashMap<String, Symbol>>,
    returns: Types,
    in_loop: bool,

    struct_defs: HashMap<String, StructDef>,

    errors: Vec<AsteriError>,
    warnings: Vec<AsteriError>,
    info: Vec<AsteriError>,
}

impl<'a> Sema<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            scopes: vec![HashMap::new()],
            returns: Types::Unit,
            in_loop: false,

            struct_defs: HashMap::new(),

            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn analyze(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Err(e) = self.check_stmt(stmt) {
                self.errors.push(e);
            }
        }
    }

    fn error(&self, line: usize, msg: impl Into<String>) -> AsteriError {
        let src_line = self
            .input
            .lines()
            .nth(line.saturating_sub(1))
            .unwrap_or(&format!("Line: [{}]", line))
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        AsteriError::new(ErrorKind::Sema, line, src_line, msg.into())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        match stmt {
            Stmt::Print(expr) => {
                self.check_expr(expr, 0)?;
                Ok(())
            }
            Stmt::Error(expr) => {
                self.check_expr(expr, 0)?;
                Ok(())
            }
            Stmt::Let { .. } => self.check_decl(stmt, false),
            Stmt::Immut { .. } => self.check_decl(stmt, true),
            Stmt::Assign { .. } => self.check_assign(stmt),
            Stmt::Fun { .. } => self.check_fun(stmt),
            Stmt::Ret { .. } => self.check_ret(stmt),
            Stmt::If { .. } => self.check_if(stmt),
            Stmt::Loop { .. } => self.check_loop(stmt),
            Stmt::While { .. } => self.check_while(stmt),
            Stmt::Break { line } => {
                if !self.in_loop {
                    return Err(self.error(*line, "'break' outside of loop"));
                }
                Ok(())
            }
            Stmt::Continue { line } => {
                if !self.in_loop {
                    return Err(self.error(*line, "'continue' outside of loop"));
                }
                Ok(())
            }
            Stmt::CBlock(_) => Ok(()),
            Stmt::Expr { expr, line } => {
                self.check_expr(expr, *line)?;
                Ok(())
            }
            Stmt::Struct {
                name,
                fields,
                methods,
                ..
            } => {
                let mut field_map = HashMap::new();
                for f in fields {
                    field_map.insert(f.name.clone(), f.tp.clone());
                }
                let mut method_map = HashMap::new();
                for m in methods {
                    let sym = Symbol::Fun {
                        params: m.params.iter().map(|(_, t)| t.clone()).collect(),
                        return_type: Box::new(m.rt_tp.clone().unwrap_or(Types::Unit)),
                    };
                    method_map.insert(m.name.clone(), sym);
                }
                self.struct_defs.insert(
                    name.clone(),
                    StructDef {
                        fields: field_map,
                        methods: method_map,
                    },
                );
                self.define(name.clone(), Symbol::Struct(name.clone()));
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.push();
                for s in stmts {
                    if let Err(e) = self.check_stmt(s) {
                        self.errors.push(e)
                    }
                }
                self.pop();
                Ok(())
            }
            Stmt::Call { name, args, line } => {
                self.check_call(name, args, *line)?;
                Ok(())
            }
            Stmt::Thunk { name, body, line } => {
                let return_type = match body {
                    LambdaBody::Expr(expr) => self.check_expr(expr, *line)?,
                    LambdaBody::Block(stmts) => {
                        self.push();
                        for stmt in stmts {
                            self.check_stmt(stmt)?;
                        }
                        self.pop();
                        Types::Unit
                    }
                };
                self.define(
                    name.clone(),
                    Symbol::Variable {
                        tp: Types::Fun {
                            params: vec![],
                            return_type: Box::new(return_type),
                        },
                        immut: false,
                    },
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_expr(&mut self, expr: &Expr, _line: usize) -> Result<Types, AsteriError> {
        match expr {
            Expr::Non => Ok(Types::Non),
            Expr::Int(_) => Ok(Types::I32),
            Expr::Float(_) => Ok(Types::F32),
            Expr::Bool(_) => Ok(Types::Bool),
            Expr::Char(_) => Ok(Types::Char),
            Expr::Str(_) => Ok(Types::Str),
            Expr::Unit => Ok(Types::Unit),
            Expr::FmtStr { .. } => Ok(Types::Str),
            Expr::Binary {
                left,
                op,
                right,
                line,
            } => self.check_binary(left, op, right, *line),
            Expr::Unary { op, expr, line } => self.check_unary(op, expr, *line),
            Expr::Reference(expr) => {
                let inner = self.check_expr(expr, _line)?;
                match inner {
                    Types::Pointer {
                        inner: ptr_inner,
                        depth,
                    } => Ok(Types::Pointer {
                        inner: ptr_inner,
                        depth: depth + 1,
                    }),
                    Types::OptionPointer {
                        inner: ptr_inner,
                        depth,
                    } => Ok(Types::Pointer {
                        inner: ptr_inner,
                        depth: depth + 1,
                    }),
                    other => Ok(Types::Pointer {
                        inner: Box::new(other),
                        depth: 1,
                    }),
                }
            }
            Expr::Dereference {
                expr,
                depth: deref_depth,
                line,
            } => {
                let tp = self.check_expr(expr, *line)?;
                match tp {
                    Types::Pointer { inner, depth } | Types::OptionPointer { inner, depth } => {
                        if depth < *deref_depth {
                            return Err(
                                self.error(*line, "dereference depth exceeds pointer depth")
                            );
                        }
                        if depth == *deref_depth {
                            Ok(*inner)
                        } else {
                            Ok(Types::Pointer {
                                inner,
                                depth: depth - deref_depth,
                            })
                        }
                    }
                    _ => Err(self.error(*line, "dereference requires a pointer type")),
                }
            }
            Expr::Id { name, line } => match self.lookup(name) {
                Some(Symbol::Variable { tp, immut: _ }) => Ok(tp.clone()),
                Some(_) => Err(self.error(*line, format!("'{}' is not a variable", name))),
                None => Err(self.error(*line, format!("'{}' is an undefined variable", name))),
            },
            Expr::Call { name, args, line } => self.check_call(name, args, *line),
            Expr::Lambda { params, body, line } => {
                let param_types: Vec<Types> = params.iter().map(|_| Types::I64).collect();
                let return_type = match body {
                    LambdaBody::Expr(expr) => {
                        self.push();
                        for (name, tp) in params.iter().zip(&param_types) {
                            self.define(
                                name.clone(),
                                Symbol::Variable {
                                    tp: tp.clone(),
                                    immut: false,
                                },
                            );
                        }
                        let result = self.check_expr(expr, *line)?;
                        self.pop();
                        result
                    }
                    LambdaBody::Block(stmts) => {
                        self.push();
                        let prev_returns = self.returns.clone();
                        self.returns = Types::Unit;
                        for (name, tp) in params.iter().zip(&param_types) {
                            self.define(
                                name.clone(),
                                Symbol::Variable {
                                    tp: tp.clone(),
                                    immut: false,
                                },
                            );
                        }
                        for stmt in stmts {
                            self.check_stmt(stmt)?;
                        }
                        self.returns = prev_returns;
                        self.pop();
                        Types::Unit
                    }
                };
                Ok(Types::Fun {
                    params: param_types,
                    return_type: Box::new(return_type),
                })
            }

            Expr::Cast { expr, target, line } => {
                let src = self.check_expr(expr, *line)?;
                if is_castable(&src, target) {
                    Ok(target.clone())
                } else {
                    Err(self.error(
                        *line,
                        format!(
                            "cannot cast '{}' to '{}'",
                            get_type_name(&src),
                            get_type_name(target)
                        ),
                    ))
                }
            }

            Expr::StructLit { name, fields, line } => {
                let field_types = self
                    .struct_defs
                    .get(name)
                    .map(|def| def.fields.clone())
                    .ok_or_else(|| self.error(*line, format!("undefined struct '{}'", name)))?;

                for (fname, fval) in fields {
                    let field_type = field_types.get(fname).ok_or_else(|| {
                        self.error(*line, format!("struct '{}' has no field '{}'", name, fname))
                    })?;
                    let val_type = self.check_expr(fval, *line)?;
                    if !is_compatible(field_type, &val_type) {
                        return Err(self.error(
                            *line,
                            format!(
                                "field '{}' type mismatch: expected {}, got {}",
                                fname,
                                get_type_name(field_type),
                                get_type_name(&val_type)
                            ),
                        ));
                    }
                }
                Ok(Types::StructInst(name.clone()))
            }
            Expr::MethodCall {
                object,
                method,
                args,
                line,
            } => {
                let obj_type = self.check_expr(object, *line)?;
                if let Types::StructInst(struct_name) = &obj_type {
                    let (params, return_type) = {
                        let def = self.struct_defs.get(struct_name).ok_or_else(|| {
                            self.error(*line, format!("undefined struct '{}'", struct_name))
                        })?;
                        let sym = def.methods.get(method).ok_or_else(|| {
                            self.error(
                                *line,
                                format!("struct '{}' has no method '{}'", struct_name, method),
                            )
                        })?;
                        if let Symbol::Fun {
                            params,
                            return_type,
                        } = sym
                        {
                            (params.clone(), return_type.clone())
                        } else {
                            return Err(self.error(*line, format!("'{}' is not a method", method)));
                        }
                    };

                    if args.len() != params.len() {
                        return Err(self.error(
                            *line,
                            format!(
                                "method '{}' expects {} arguments, got {}",
                                method,
                                params.len(),
                                args.len()
                            ),
                        ));
                    }
                    for (arg, param_type) in args.iter().zip(params.iter()) {
                        let arg_type = self.check_expr(arg, *line)?;
                        if !is_compatible(param_type, &arg_type) {
                            return Err(self.error(
                                *line,
                                format!("argument type mismatch in method '{}'", method),
                            ));
                        }
                    }
                    Ok(*return_type)
                } else {
                    Err(self.error(*line, "method call requires a struct instance"))
                }
            }
        }
    }

    fn check_decl(&mut self, stmt: &Stmt, immut: bool) -> Result<(), AsteriError> {
        let (name, tp, value, line) = match stmt {
            Stmt::Let {
                name,
                tp,
                value,
                line,
                ..
            } => (name, tp, value, line),
            Stmt::Immut {
                name,
                tp,
                value,
                line,
                ..
            } => (name, tp, value, line),
            _ => unreachable!(),
        };

        if self.scopes.last().unwrap().contains_key(name.as_str()) {
            return Err(self.error(
                *line,
                format!("'{}' is already declared in this scope", name),
            ));
        }

        let res_tp = match (tp, value) {
            (Some(t), None) => t.clone(),
            (Some(t), Some(expr)) => {
                let init_tp = self.check_expr(expr, *line)?;
                if !is_compatible(t, &init_tp) {
                    return Err(self.error(
                        *line,
                        format!(
                            "type mismatch: declared '{}' but initialized with '{}'",
                            get_type_name(t),
                            get_type_name(&init_tp)
                        ),
                    ));
                }
                t.clone()
            }
            (None, Some(expr)) => self.check_expr(expr, *line)?,
            (None, None) => {
                return Err(self.error(
                    *line,
                    format!(
                        "cannot infer type of '{}', need type annotation or initializer",
                        name
                    ),
                ))
            }
        };

        self.define(name.clone(), Symbol::Variable { tp: res_tp, immut });
        Ok(())
    }

    fn check_assign(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Assign {
            target,
            value,
            line,
        } = stmt
        {
            let target_tp = self.check_lvalue(target, *line)?;
            let val_tp = self.check_expr(value, *line)?;
            if !is_compatible(&target_tp, &val_tp) {
                return Err(self.error(
                    *line,
                    format!(
                        "return type mismatch, expected: '{}', got: '{}'",
                        get_type_name(&target_tp),
                        get_type_name(&val_tp)
                    ),
                ));
            }
        }
        Ok(())
    }

    fn check_lvalue(&mut self, expr: &Expr, line: usize) -> Result<Types, AsteriError> {
        match expr {
            Expr::Id { name, .. } => match self.lookup(name) {
                Some(Symbol::Variable { tp, immut: false }) => Ok(tp.clone()),
                Some(Symbol::Variable { immut: true, .. }) => {
                    Err(self.error(line, format!("'{}' is immutable", name)))
                }
                _ => Err(self.error(line, format!("'{}' is not a mutable variable", name))),
            },
            Expr::Dereference {
                expr,
                depth: deref_depth,
                line,
            } => {
                let tp = self.check_lvalue(expr, *line)?;
                match &tp {
                    Types::Pointer { inner, depth } | Types::OptionPointer { inner, depth } => {
                        if *depth < *deref_depth {
                            return Err(
                                self.error(*line, "dereference depth surpasse pointer depth")
                            );
                        }
                        if *depth == *deref_depth {
                            Ok(*inner.clone())
                        } else {
                            Ok(Types::Pointer {
                                inner: inner.clone(),
                                depth: depth - deref_depth,
                            })
                        }
                    }
                    _ => Err(self.error(*line, "cannot dereference non-pointer type")),
                }
            }
            _ => Err(self.error(line, "invalid assignment target")),
        }
    }

    fn check_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        line: usize,
    ) -> Result<Types, AsteriError> {
        let lt = self.check_expr(left, line)?;
        let rt = self.check_expr(right, line)?;
        if !is_compatible(&lt, &rt) {
            return Err(self.error(
                line,
                format!(
                    "({} {} {}): type mismatch in binary opration",
                    get_type_name(&lt),
                    to_op(op),
                    get_type_name(&rt)
                ),
            ));
        }

        match op {
            BinaryOp::Eql
            | BinaryOp::Neq
            | BinaryOp::LessTh
            | BinaryOp::GreaTh
            | BinaryOp::LessOr
            | BinaryOp::GreaOr => Ok(Types::Bool),

            _ => Ok(lt),
        }
    }

    fn check_unary(
        &mut self,
        op: &UnaryOp,
        expr: &Expr,
        line: usize,
    ) -> Result<Types, AsteriError> {
        let inner = self.check_expr(expr, line)?;
        match op {
            UnaryOp::Minus | UnaryOp::BitNot => {
                if !is_numeric(&inner) {
                    return Err(self.error(line, "unary operator requires numirc operand"));
                }
                Ok(inner)
            }
            UnaryOp::Not => {
                if inner != Types::Bool {
                    return Err(self.error(line, "logical not requires boolean operand"));
                }
                Ok(Types::Bool)
            }
        }
    }

    fn check_fun(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        let Stmt::Fun {
            rt_tp,
            name,
            params,
            body,
            ..
        } = stmt
        else {
            unreachable!()
        };
        self.define(
            name.clone(),
            Symbol::Fun {
                params: params.iter().map(|(_, t)| t.clone()).collect(),
                return_type: Box::new(rt_tp.clone().unwrap_or(Types::Unit)),
            },
        );
        let previous = self.returns.clone();
        self.returns = rt_tp.clone().unwrap_or(Types::Unit);
        self.push();
        for (p_name, p_type) in params {
            self.define(
                p_name.clone(),
                Symbol::Variable {
                    tp: p_type.clone(),
                    immut: false,
                },
            );
        }
        for stmt in body {
            if let Err(e) = self.check_stmt(stmt) {
                self.errors.push(e)
            }
        }
        self.pop();
        self.returns = previous;
        Ok(())
    }

    fn check_ret(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Ret { expr, line } = stmt {
            match expr {
                Some(e) => {
                    let t = self.check_expr(e, *line)?;
                    if self.returns == Types::Unit {
                        return Err(self.error(*line, "unexpected return value in void function"));
                    }
                    if !is_compatible(&self.returns, &t) {
                        return Err(self.error(
                            *line,
                            format!(
                                "return type mismatch, expected: {}, got: {}",
                                get_type_name(&self.returns),
                                get_type_name(&t)
                            ),
                        ));
                    }
                }
                None => {
                    if self.returns != Types::Unit {
                        return Err(self.error(*line, "missing returns value"));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_if(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::If {
            condition,
            body,
            else_branch,
            line,
        } = stmt
        {
            let cd_tp = self.check_expr(condition, *line)?;
            if cd_tp != Types::Bool {
                return Err(self.error(*line, "if condition must be a boolean"));
            }
            self.check_scoped_body(body);
            if let Some(else_stmt) = else_branch {
                self.push();
                if let Err(e) = self.check_stmt(else_stmt) {
                    self.errors.push(e)
                }
                self.pop();
            }
        }
        Ok(())
    }

    fn check_while(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::While {
            condition,
            body,
            line,
        } = stmt
        {
            let cd_tp = self.check_expr(condition, *line)?;
            if cd_tp != Types::Bool {
                return Err(self.error(*line, "while condition must be a boolean"));
            }
            let prev = self.in_loop;
            self.in_loop = true;
            self.check_scoped_body(body);
            self.in_loop = prev;
        }
        Ok(())
    }

    fn check_loop(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Loop { body } = stmt {
            let prev = self.in_loop;
            self.in_loop = true;
            self.check_scoped_body(body);
            self.in_loop = prev;
        }
        Ok(())
    }

    fn check_scoped_body(&mut self, body: &[Stmt]) {
        self.push();
        for stmt in body {
            if let Err(e) = self.check_stmt(stmt) {
                self.errors.push(e);
            }
        }
        self.pop();
    }

    fn check_call(&mut self, name: &str, args: &[Expr], line: usize) -> Result<Types, AsteriError> {
        let (params, return_type) = match self.lookup(name) {
            Some(Symbol::Fun {
                params,
                return_type,
            }) => (params.clone(), return_type.clone()),

            Some(Symbol::Variable {
                tp:
                    Types::Fun {
                        params,
                        return_type,
                    },
                ..
            }) => (params.clone(), return_type.clone()),

            Some(_) => return Err(self.error(line, format!("'{}' is not a function", name))),

            None => return Err(self.error(line, format!("'{}' is undefined", name))),
        };
        if args.len() != params.len() {
            return Err(self.error(
                line,
                format!(
                    "'{}' expects {} arguments, got {}",
                    name,
                    params.len(),
                    args.len()
                ),
            ));
        }
        for (arg, param_tp) in args.iter().zip(params.iter()) {
            let arg_tp = self.check_expr(arg, line)?;
            if !is_compatible(param_tp, &arg_tp) {
                return Err(self.error(
                    line,
                    format!(
                        "argument type mismatch: expected {}, got {}",
                        get_type_name(param_tp),
                        get_type_name(&arg_tp)
                    ),
                ));
            }
        }
        Ok(*return_type)
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, symbol: Symbol) {
        self.scopes.last_mut().unwrap().insert(name, symbol);
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn take_all(self) -> (Vec<AsteriError>, Vec<AsteriError>, Vec<AsteriError>) {
        (self.errors, self.warnings, self.info)
    }
}

fn is_compatible(exp: &Types, got: &Types) -> bool {
    if exp == got {
        return true;
    }
    let same_name = |a: &Types, b: &Types| -> bool {
        match (a, b) {
            (Types::Named(na), Types::Named(nb)) => na == nb,
            (Types::StructInst(na), Types::StructInst(nb)) => na == nb,
            (Types::Named(na), Types::StructInst(nb)) => na == nb,
            (Types::StructInst(na), Types::Named(nb)) => na == nb,
            _ => false,
        }
    };

    if same_name(exp, got) {
        return true;
    }

    if matches!(got, Types::Non) && matches!(exp, Types::OptionPointer { .. }) {
        return true;
    }

    if let (
        Types::OptionPointer {
            inner: e,
            depth: ed,
        },
        Types::Pointer {
            inner: g,
            depth: gd,
        },
    ) = (exp.clone(), got.clone())
    {
        if ed == gd && is_compatible(&e, &g) {
            return true;
        }
    }

    let int_types = [
        Types::I8,
        Types::I16,
        Types::I32,
        Types::I64,
        Types::U8,
        Types::U16,
        Types::U32,
        Types::U64,
    ];

    let flt_types = [Types::F32, Types::F64];

    let exp_int = int_types.contains(exp);
    let got_int = int_types.contains(got);
    let exp_flt = flt_types.contains(exp);
    let got_flt = flt_types.contains(got);

    (exp_int && got_int) || (exp_flt && got_flt) || (exp_flt && got_int)
}

fn get_type_name(n: &Types) -> Cow<'static, str> {
    match n {
        Types::Non => "None".into(),
        Types::I8 => "i8".into(),
        Types::I16 => "i16".into(),
        Types::I32 => "i32".into(),
        Types::I64 => "i64".into(),
        Types::U8 => "u8".into(),
        Types::U16 => "u16".into(),
        Types::U32 => "u32".into(),
        Types::U64 => "u64".into(),
        Types::F32 => "f32".into(),
        Types::F64 => "f64".into(),
        Types::String => "string".into(),
        Types::Bool => "bool".into(),
        Types::Char => "char".into(),
        Types::File => "file".into(),
        Types::Matrix3x3 => "matrix3x3".into(),
        Types::Matrix4x4 => "matrix4x4".into(),
        Types::Vector2 => "vector2".into(),
        Types::Vector3 => "vector3".into(),
        Types::Str => "str".into(),
        Types::Cstr => "cstr".into(),
        Types::Istr => "istr".into(),
        Types::Unit => "()".into(),
        Types::Pointer { .. } => "pointer".into(),
        Types::OptionPointer { .. } => "optional pointer".into(),
        Types::Fun { .. } => "function".into(),
        Types::Named(name) | Types::StructInst(name) => Cow::Owned(name.clone()),
    }
}

fn to_op(s: &BinaryOp) -> &'static str {
    match s {
        BinaryOp::Add => "+",
        BinaryOp::BitAnd => "BitAnd",
        BinaryOp::BitOr => "BitOr",
        BinaryOp::BitXor => "BitXor",
        BinaryOp::Div => "/",
        BinaryOp::Eql => "==",
        BinaryOp::GreaOr => ">=",
        BinaryOp::GreaTh => ">",
        BinaryOp::LessOr => "<=",
        BinaryOp::LessTh => "<",
        BinaryOp::LogicAnd => "&&",
        BinaryOp::LogicOr => "||",
        BinaryOp::Mul => "*",
        BinaryOp::Neq => "!=",
        BinaryOp::ShiftLeft => "<<",
        BinaryOp::ShiftRight => ">>",
        BinaryOp::Sub => "-",
    }
}

fn is_numeric(tp: &Types) -> bool {
    matches!(
        tp,
        Types::I8
            | Types::I16
            | Types::I32
            | Types::I64
            | Types::U8
            | Types::U16
            | Types::U32
            | Types::U64
            | Types::F32
            | Types::F64
    )
}

fn is_castable(src: &Types, target: &Types) -> bool {
    use Types::*;

    match (src, target) {
        (
            I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | Char,
            I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | Char,
        ) => true,

        (F32, F64) | (F64, F32) => true,

        (I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64, F32 | F64) => true,
        (F32 | F64, I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64) => true,

        (Pointer { .. }, Pointer { .. }) => true,
        (Pointer { .. }, OptionPointer { .. }) => true,
        (OptionPointer { .. }, Pointer { .. }) => true,

        _ => false,
    }
}
