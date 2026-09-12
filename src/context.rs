use an_inkwell::{Context as LLVMContext, Error};

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

    pub fn module(&self, name: &str) -> Result<Module<'_>, Error> {
        self.raw
            .module(name)
            .map(|module| Module::from_raw(self, module))
    }

    pub(crate) fn as_raw(&self) -> &LLVMContext {
        &self.raw
    }
}
