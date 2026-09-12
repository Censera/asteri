use an_inkwell::Module as LLVMModule;

use crate::Context;

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

    pub fn as_ir(&self) -> String {
        self.raw.as_ir()
    }

    pub(crate) fn as_raw(&self) -> &LLVMModule<'ctx> {
        &self.raw
    }
}
