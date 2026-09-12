use inkwell::builder::BuilderError;
use inkwell::values::FunctionValue;

use crate::Context;

pub struct Handler<'ctx> {
    builder: inkwell::builder::Builder<'ctx>,
}

impl<'ctx> Handler<'ctx> {
    pub(crate) fn new(context: &'ctx Context, function: FunctionValue<'ctx>) -> Self {
        let builder = context.as_raw().create_builder();
        let block = context.as_raw().append_basic_block(function, "entry");
        builder.position_at_end(block);
        Self { builder }
    }

    pub fn return_void(&self) -> Result<(), BuilderError> {
        self.builder.build_return(None).map(|_| ())
    }

    pub fn return_i32(&self, value: i32) -> Result<(), BuilderError> {
        let value = self.builder.get_insert_block().unwrap().get_context().i32_type().const_int(value as u64, true);
        self.builder.build_return(Some(&value)).map(|_| ())
    }

    pub fn return_i64(&self, value: i64) -> Result<(), BuilderError> {
        let value = self.builder.get_insert_block().unwrap().get_context().i64_type().const_int(value as u64, true);
        self.builder.build_return(Some(&value)).map(|_| ())
    }
}
