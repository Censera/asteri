use inkwell::module::Module as LLVMModule;

use crate::{Context, Function};

pub struct Module<'ctx> {
    context: &'ctx Context,
    raw: LLVMModule<'ctx>,
}

impl<'ctx> Module<'ctx> {
    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMModule<'ctx>) -> Self {
        Self { context, raw }
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn function(&self, name: &str) -> Option<Function<'ctx>> {
        self.raw.get_function(name).map(Function::from_raw)
    }

    pub fn as_ir(&self) -> String {
        self.raw.print_to_string().to_string()
    }

    pub(crate) fn as_raw(&self) -> &LLVMModule<'ctx> {
        &self.raw
    }
}
