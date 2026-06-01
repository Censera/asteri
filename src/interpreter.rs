// use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
// use std::collections::HashMap;
//
// #[derive(Debug)]
// pub struct RuntimeError {
//     pub content: String,
// }
//
// impl std::fmt::Display for RuntimeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "[Runtime] {}", self.content)
//     }
// }
//
// impl std::error::Error for RuntimeError {}
//
// #[derive(Debug, Clone)]
// pub enum Value {
//     Null,
//     Int(isize),
//     Float(f64),
//     Bool(bool),
//     Str(String),
//     Pointer(usize),
// }
//
// pub struct Environment {
//     scopes: Vec<HashMap<String, Value>>,
// }
//
// impl Environment {
//     pub fn new() -> Self {
//         Self {
//             scopes: vec![HashMap::new()],
//         }
//     }
//
//     pub fn push_scope(&mut self) {
//         self.scopes.push(HashMap::new());
//     }
//
//     pub fn push_pop(&mut self) {
//         self.scopes.pop();
//     }
//
//     pub fn set(&mut self, name: String, value: Value) {
//         if let Some(scope) = self.scopes.last_mut() {
//             scope.insert(name, value);
//         }
//     }
//
//     pub fn get(&self, name: &str) -> Option<&Value> {
//         for scope in self.scopes.iter().rev() {
//             if let Some(v) = scope.get(name) {
//                 return Some(v);
//             }
//         }
//         None
//     }
// }
//
// pub struct Interpreter {
//     env: Environment,
// }
//
// impl Interpreter {
//     pub fn new() -> Self {
//         Self {
//             env: Environment::new(),
//         }
//     }
//
//     pub fn run(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
//         for stmt in stmts {
//             self.exec_stmt(stmt)?;
//         }
//         Ok(())
//     }
//
//     fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
//         match stmt {
//             Stmt::Print(expr) => {
//                 let v = self.eval_expr(expr)?;
//                 println!("{:?}", v);
//                 Ok(())
//             }
//             _ => Err(self.error("Unimplemented expression")),
//         }
//     }
//
//     fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
//         match expr {
//             Expr::Int(n) => Ok(Value::Int(*n)),
//             _ => Err(self.error("Unimplemented statement")),
//         }
//     }
//
//     fn error(&self, content: &str) -> RuntimeError {
//         RuntimeError {
//             content: content.to_string(),
//         }
//     }
// }
