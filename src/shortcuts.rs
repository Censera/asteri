use inkwell::builder::BuilderError;

use crate::{Context, Function, Handler, Module};

pub fn void_function<'ctx>(module: &'ctx Module<'ctx>, name: &str) -> Function<'ctx> {
    Function::void(module, name)
}

pub fn i32_function<'ctx>(module: &'ctx Module<'ctx>, name: &str) -> Function<'ctx> {
    Function::i32(module, name)
}

pub fn i64_function<'ctx>(module: &'ctx Module<'ctx>, name: &str) -> Function<'ctx> {
    Function::i64(module, name)
}

pub fn handler<'ctx>(function: &Function<'ctx>, context: &'ctx Context) -> Handler<'ctx> {
    function.handler(context)
}

pub fn return_void(handler: &Handler<'_>) -> Result<(), BuilderError> {
    handler.return_void()
}

pub fn return_i32(handler: &Handler<'_>, value: i32) -> Result<(), BuilderError> {
    handler.return_i32(value)
}

pub fn return_i64(handler: &Handler<'_>, value: i64) -> Result<(), BuilderError> {
    handler.return_i64(value)
}
