use inkwell::module::Module;
use inkwell::values::FunctionValue;

use crate::{Context, Handler};

pub struct Function<'ctx> {
    value: FunctionValue<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn void(module: &Module<'ctx>, name: &str) -> Self {
        let context = module.get_context();
        let function_type = context.void_type().fn_type(&[], false);
        Self::from_raw(module.add_function(name, function_type, None))
    }

    pub fn i32(module: &Module<'ctx>, name: &str) -> Self {
        let context = module.get_context();
        let function_type = context.i32_type().fn_type(&[], false);
        Self::from_raw(module.add_function(name, function_type, None))
    }

    pub fn i64(module: &Module<'ctx>, name: &str) -> Self {
        let context = module.get_context();
        let function_type = context.i64_type().fn_type(&[], false);
        Self::from_raw(module.add_function(name, function_type, None))
    }

    pub(crate) fn from_raw(value: FunctionValue<'ctx>) -> Self {
        Self { value }
    }

    pub fn handler(&self, context: &'ctx Context) -> Handler<'ctx> {
        Handler::new(context, self.value)
    }

    pub fn name(&self) -> String {
        self.value
            .get_name()
            .to_string_lossy()
            .into_owned()
    }

    pub(crate) fn as_raw(&self) -> FunctionValue<'ctx> {
        self.value
    }
}
