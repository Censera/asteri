use inkwell::context::Context as LLVMContext;

use crate::Module;

pub struct Context {
    raw: LLVMContext,
}

impl Context {
    pub fn create() -> Self {
        Self {
            raw: LLVMContext::create(),
        }
    }

    pub fn module(&self, name: &str) -> Module<'_> {
        Module::from_raw(self, self.raw.create_module(name))
    }

    pub(crate) fn as_raw(&self) -> &LLVMContext {
        &self.raw
    }
}
