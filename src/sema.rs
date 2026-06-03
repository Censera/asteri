use crate::ast::{BinaryOp, Expr, Stmt, Types};
use crate::error::{AsteriError, ErrorKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable {
        tp: Types,
        immut: bool,
    },

    Fun {
        params: Vec<Types>,
        return_type: Option<Types>,
    },

    Struct(String),
}

pub struct Sema<'a> {
    input: &'a str,
    scopes: Vec<HashMap<String, Symbol>>,
    errors: Vec<AsteriError>,
    warnings: Vec<AsteriError>,
    info: Vec<AsteriError>,
}

impl<'a> Sema<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            scopes: vec![HashMap::new()],
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
            self.check_stmt(stmt);
        }
    }

    fn error(&self, line: usize, msg: impl Into<String>) -> AsteriError {
        AsteriError::new(ErrorKind::Parser, line, "".to_string(), msg.into())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        match stmt {
            Stmt::Print(expr) => {
                self.check_expr(expr)?;
                Ok(())
            }
            Stmt::Let { .. } => self.check_let(stmt),
            Stmt::Immut { .. } => self.check_immut(stmt),
            Stmt::Assign { .. } => self.check_assign(stmt),
            _ => Ok(()),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Types, AsteriError> {
        match expr {
            Expr::Int(_) => Ok(Types::I64),
            Expr::Float(_) => Ok(Types::F64),
            Expr::Bool(_) => Ok(Types::Bool),
            Expr::Str(_) => Ok(Types::String),
            Expr::Binary {
                left,
                op,
                right,
                line,
            } => self.check_binary(left, op, right, *line),
            Expr::Id(name) => match self.lookup(name) {
                Some(Symbol::Variable { tp, immut: _ }) => Ok(tp.clone()),
                Some(_) => Err(self.error(0, &format!("{} is not a variable", name))),
                None => Err(self.error(0, &format!("{} is an undefined variable", name))),
            },
            _ => Err(self.error(0, "Unimplemented expression")),
        }
    }

    fn check_let(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Let {
            name,
            tp,
            value,
            line,
        } = stmt
        {
            if self.scopes.last().unwrap().contains_key(name.as_str()) {
                return Err(self.error(
                    *line,
                    &format!("|{}| is already declared in this scope", name),
                ));
            }
            if let Some(expr) = value {
                self.check_expr(expr)?;
            }
            if let Some(t) = tp {
                self.define(
                    name.clone(),
                    Symbol::Variable {
                        tp: t.clone(),
                        immut: false,
                    },
                );
            } else {
                return Err(self.error(*line, "let declaration missing type"));
            }
        }
        Ok(())
    }

    fn check_immut(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Immut {
            name,
            tp,
            value,
            line,
        } = stmt
        {
            if self.scopes.last().unwrap().contains_key(name.as_str()) {
                return Err(self.error(
                    *line,
                    &format!("|{}| is already declared in this scope", name),
                ));
            }
            if let Some(expr) = value {
                self.check_expr(expr)?;
            }
            if let Some(t) = tp {
                self.define(
                    name.clone(),
                    Symbol::Variable {
                        tp: t.clone(),
                        immut: true,
                    },
                );
            } else {
                return Err(self.error(*line, "immut declaration missing type"));
            }
        }
        Ok(())
    }

    fn check_assign(&mut self, stmt: &Stmt) -> Result<(), AsteriError> {
        if let Stmt::Assign { name, value, line } = stmt {
            let var_tp = match self.lookup(name) {
                Some(Symbol::Variable { tp, immut: false }) => tp.clone(),
                Some(Symbol::Variable { tp: _, immut: true }) => {
                    return Err(self.error(0, &format!("{} is immutable", name)))
                }
                Some(_) => return Err(self.error(*line, &format!("{} is not a variable", name))),
                None => {
                    return Err(self.error(*line, &format!("{} is an undefined variable", name)))
                }
            };
            let val_tp = self.check_expr(value)?;
            if !is_compatible(&var_tp, &val_tp) {
                return Err(self.error(
                    *line,
                    &format!("Type mismatch: expected {:?}, got {:?}", var_tp, val_tp),
                ));
            }
        }
        Ok(())
    }

    fn check_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
        line: usize,
    ) -> Result<Types, AsteriError> {
        let lt = self.check_expr(left)?;
        let rt = self.check_expr(right)?;
        if !is_compatible(&lt, &rt) {
            return Err(self.error(
                line,
                &format!(
                    "Type mismatch in binary opration: {:?} {:?} {:?}",
                    lt, op, rt
                ),
            ));
        }
        Ok(lt)
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
