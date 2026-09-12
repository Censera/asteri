use an_inkwell::{Builder, Type, Value};

use crate::Context;

pub struct Handler<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

impl<'ctx> Handler<'ctx> {
    pub(crate) fn new(
        context: &'ctx Context,
        function: &an_inkwell::Function<'ctx>,
    ) -> Result<Self, an_inkwell::Error> {
        let builder = context.as_raw().builder()?;
        let block = function.block("entry")?;
        builder.position(&block)?;
        Ok(Self { context, builder })
    }

    pub fn return_void(&self) -> Result<Value<'ctx>, an_inkwell::Error> {
        self.builder.ret_void()
    }

    pub fn return_i32(&self, value: i32) -> Result<Value<'ctx>, an_inkwell::Error> {
        let ty = Type::i32(self.context.as_raw());
        let value = Value::integer(&ty, value as u64, true);
        self.builder.ret(&value)
    }

    pub fn return_i64(&self, value: i64) -> Result<Value<'ctx>, an_inkwell::Error> {
        let ty = Type::i64(self.context.as_raw());
        let value = Value::integer(&ty, value as u64, true);
        self.builder.ret(&value)
    }
}
