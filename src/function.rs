use an_inkwell::{Function as LLVMFunction, Type};

use crate::{Context, Handler, Module};

pub struct Function<'ctx> {
    value: LLVMFunction<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn void(module: &Module<'ctx>, name: &str) -> Result<Self, an_inkwell::Error> {
        let context = module.context().as_raw();
        let function_type = Type::function(&Type::void(context), &[], false)?;
        module
            .as_raw()
            .function(name, &function_type)
            .map(Self::from_raw)
    }

    pub fn i32(module: &Module<'ctx>, name: &str) -> Result<Self, an_inkwell::Error> {
        let context = module.context().as_raw();
        let return_type = Type::i32(context);
        let function_type = Type::function(&return_type, &[], false)?;
        module
            .as_raw()
            .function(name, &function_type)
            .map(Self::from_raw)
    }

    pub fn i64(module: &Module<'ctx>, name: &str) -> Result<Self, an_inkwell::Error> {
        let context = module.context().as_raw();
        let return_type = Type::i64(context);
        let function_type = Type::function(&return_type, &[], false)?;
        module
            .as_raw()
            .function(name, &function_type)
            .map(Self::from_raw)
    }

    pub(crate) fn from_raw(value: LLVMFunction<'ctx>) -> Self {
        Self { value }
    }

    pub fn handler(&self, context: &'ctx Context) -> Result<Handler<'ctx>, an_inkwell::Error> {
        Handler::new(context, &self.value)
    }

    pub fn name(&self) -> String {
        self.value.name()
    }
}
