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
        if !std::ptr::eq(context.as_raw(), function.context()) {
            return Err(an_inkwell::Error::DifferentContext);
        }

        let builder = context.as_raw().builder()?;
        let block = function.block("entry")?;
        builder.position(&block)?;
        Ok(Self { context, builder })
    }

    pub fn return_void(&self) -> Result<(), an_inkwell::Error> {
        self.builder.ret_void();
        Ok(())
    }

    pub fn return_i32(&self, value: i32) -> Result<(), an_inkwell::Error> {
        let ty = Type::i32(self.context.as_raw());
        let value = Value::integer(&ty, value as u64, true);
        self.builder.ret(&value).map(|_| ())
    }

    pub fn return_i64(&self, value: i64) -> Result<(), an_inkwell::Error> {
        let ty = Type::i64(self.context.as_raw());
        let value = Value::integer(&ty, value as u64, true);
        self.builder.ret(&value).map(|_| ())
    }
}
