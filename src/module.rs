use an_inkwell::{Module as LLVMModule, Type};

use crate::{Context, Function};

pub struct Module<'ctx> {
    context: &'ctx Context,
    raw: LLVMModule<'ctx>,
}

impl<'ctx> Module<'ctx> {
    pub(crate) fn from_raw(raw: LLVMModule<'ctx>) -> Self {
        let context = raw.context();
        Self { context, raw }
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn function(&self, name: &str) -> Result<Function<'ctx>, an_inkwell::Error> {
        let function_type = Type::void(self.context.as_raw());
        self.raw
            .function(name, &function_type)
            .map(Function::from_raw)
    }

    pub fn as_ir(&self) -> String {
        self.raw.as_ir()
    }

    pub(crate) fn as_raw(&self) -> &LLVMModule<'ctx> {
        &self.raw
    }
}
