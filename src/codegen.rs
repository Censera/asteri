use crate::ast::{BinaryOp, Expr, Stmt, Types};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::IntPredicate;

use std::collections::HashMap;

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let variables = HashMap::new();

        Self {
            context,
            module,
            builder,
            variables,
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        let printf_tp = self.context.i32_type().fn_type(
            &[self.context.ptr_type(AddressSpace::default()).into()],
            true,
        );

        self.module.add_function("printf", printf_tp, None);

        for stmt in stmts {
            self.cmpl_stmt(stmt)?
        }
        Ok(())
    }

    fn llvm_tp(&self, tp: &Types) -> BasicTypeEnum<'ctx> {
        match tp {
            Types::Unit => self.context.i32_type().into(),
            Types::I8 => self.context.i8_type().into(),
            Types::I16 => self.context.i16_type().into(),
            Types::I32 => self.context.i32_type().into(),
            Types::I64 => self.context.i64_type().into(),
            Types::U8 => self.context.i8_type().into(),
            Types::U16 => self.context.i16_type().into(),
            Types::U32 => self.context.i32_type().into(),
            Types::U64 => self.context.i64_type().into(),
            Types::F32 => self.context.f32_type().into(),
            Types::F64 => self.context.f64_type().into(),
            Types::Bool => self.context.bool_type().into(),
            Types::Char => self.context.i32_type().into(),

            Types::Pointer { .. } => self.context.ptr_type(AddressSpace::default()).into(),
            Types::Str | Types::String => self.context.ptr_type(AddressSpace::default()).into(),
            _ => unimplemented!("type {:?}", tp),
        }
    }

    fn cmpl_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if let Some(expr) = value {
                    let v = self.cmpl_expr(expr)?;
                    let alc = self
                        .builder
                        .build_alloca(v.get_type(), name)
                        .map_err(map_err)?;
                    self.builder.build_store(alc, v).map_err(map_err)?;
                    self.variables.insert(name.clone(), (alc, v.get_type()));
                }
                Ok(())
            }
            Stmt::Expr { expr, .. } => {
                self.cmpl_expr(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let printf_fn = self
                    .module
                    .get_function("printf")
                    .ok_or_else(|| "printf not found".to_string())?;

                let v = self.cmpl_expr(expr)?;

                let fmt_str = match v {
                    BasicValueEnum::IntValue(_) => self
                        .builder
                        .build_global_string_ptr("%d\n", "fmt")
                        .map_err(map_err)?
                        .as_pointer_value(),
                    _ => self
                        .builder
                        .build_global_string_ptr("%s\n", "fmt")
                        .map_err(map_err)?
                        .as_pointer_value(),
                };

                self.builder
                    .build_call(printf_fn, &[fmt_str.into(), v.into()], "printf")
                    .map_err(map_err)?;
                Ok(())
            }
            Stmt::Ret { expr, .. } => {
                match expr {
                    Some(e) => {
                        let v = self.cmpl_expr(e)?;
                        self.builder.build_return(Some(&v)).map_err(map_err)?;
                    }
                    None => {
                        self.builder.build_return(None).map_err(map_err)?;
                    }
                }
                Ok(())
            }
            Stmt::Fun {
                name,
                rt_tp,
                params,
                body,
                ..
            } => {
                let fn_tp = self.fn_tp(name, rt_tp, params);
                let func = self.module.add_function(name, fn_tp, None);
                let entry = self.context.append_basic_block(func, "entry");
                self.builder.position_at_end(entry);

                for (i, (p_name, _)) in params.iter().enumerate() {
                    let param = func.get_nth_param(i as u32).unwrap();
                    let alc = self
                        .builder
                        .build_alloca(param.get_type(), p_name)
                        .map_err(map_err)?;
                    self.builder.build_store(alc, param).map_err(map_err)?;
                    self.variables
                        .insert(p_name.clone(), (alc, param.get_type()));
                }

                for s in body {
                    self.cmpl_stmt(s)?;
                }

                if let Some(last_block) = func.get_last_basic_block() {
                    if last_block.get_terminator().is_none() {
                        self.builder.position_at_end(last_block);
                        if name == "main" {
                            let zero = self.context.i32_type().const_int(0, false);
                            let ret_val = BasicValueEnum::IntValue(zero);
                            self.builder.build_return(Some(&ret_val)).map_err(map_err)?;
                        } else {
                            self.builder.build_return(None).map_err(map_err)?;
                        }
                    }
                }

                Ok(())
            }
            _ => Err(format!("unimplemented statement {:?}", stmt)),
        }
    }

    fn cmpl_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::Int(n) => {
                let tp = self.context.i32_type();
                Ok(tp.const_int(*n as u64, true).into())
            }
            Expr::Float(f) => {
                let tp = self.context.f32_type();
                Ok(tp.const_float(*f).into())
            }
            Expr::Bool(b) => {
                let tp = self.context.bool_type();
                Ok(tp.const_int(*b as u64, false).into())
            }
            Expr::Char(c) => {
                let tp = self.context.i32_type();
                Ok(tp.const_int(*c as u64, false).into())
            }
            Expr::Str(s) => {
                let str_v = self
                    .builder
                    .build_global_string_ptr(s, ".str")
                    .map_err(map_err)?;
                Ok(str_v.as_pointer_value().into())
            }
            Expr::Id { name, .. } => {
                let (ptr, tp) = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("undefined variable '{}'", name))?;
                let v = self.builder.build_load(*tp, *ptr, name).map_err(map_err)?;
                Ok(v)
            }
            Expr::Binary {
                left, op, right, ..
            } => {
                let l = self.cmpl_expr(left)?;
                let r = self.cmpl_expr(right)?;
                self.cmpl_bin_op(l, op, r)
            }

            Expr::Cast { expr, target, .. } => {
                let src_v = self.cmpl_expr(expr)?;
                let target_tp = self.llvm_tp(target);
                self.cast_value(src_v, target_tp)
            }

            // TODO: Call
            _ => Err(format!("unimplemented expr: {:?}", expr)),
        }
    }

    fn cmpl_bin_op(
        &self,
        l: BasicValueEnum<'ctx>,
        o: &BinaryOp,
        r: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let l_int: IntValue = l.into_int_value();
        let r_int: IntValue = r.into_int_value();
        let result = match o {
            BinaryOp::Add => self
                .builder
                .build_int_add(l_int, r_int, "add")
                .map_err(map_err)?,
            BinaryOp::Sub => self
                .builder
                .build_int_sub(l_int, r_int, "sub")
                .map_err(map_err)?,
            BinaryOp::Mul => self
                .builder
                .build_int_mul(l_int, r_int, "mul")
                .map_err(map_err)?,
            BinaryOp::Div => self
                .builder
                .build_int_signed_div(l_int, r_int, "div")
                .map_err(map_err)?,
            BinaryOp::Eql => self
                .builder
                .build_int_compare(IntPredicate::EQ, l_int, r_int, "eq")
                .map_err(map_err)?,
            BinaryOp::Neq => self
                .builder
                .build_int_compare(IntPredicate::NE, l_int, r_int, "neq")
                .map_err(map_err)?,
            BinaryOp::LessTh => self
                .builder
                .build_int_compare(IntPredicate::SLT, l_int, r_int, "lt")
                .map_err(map_err)?,
            BinaryOp::GreaTh => self
                .builder
                .build_int_compare(IntPredicate::SGT, l_int, r_int, "gt")
                .map_err(map_err)?,
            _ => return Err(format!("unimplemented binop: {:?}", o)),
        };
        Ok(result.into())
    }

    fn fn_tp(
        &self,
        name: &str,
        rt_tp: &Option<Types>,
        params: &[(String, Types)],
    ) -> FunctionType<'ctx> {
        let param_tps: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|(_, tp)| self.llvm_tp(tp).into())
            .collect();

        match rt_tp {
            Some(tp) => {
                let rt = self.llvm_tp(tp);
                rt.fn_type(&param_tps, false)
            }
            None if name == "main" => self.context.i32_type().fn_type(&param_tps, false),
            None => self.context.void_type().fn_type(&param_tps, false),
        }
    }

    fn cast_value(
        &self,
        value: BasicValueEnum<'ctx>,
        target: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match (value, target) {
            (BasicValueEnum::IntValue(v), t) if t.is_int_type() => {
                let target_type = t.into_int_type();
                let src_bits = v.get_type().get_bit_width();
                let dst_bits = target_type.get_bit_width();
                if dst_bits > src_bits {
                    Ok(self
                        .builder
                        .build_int_s_extend_or_bit_cast(v, target_type, "ext")
                        .map_err(map_err)?
                        .into())
                } else if dst_bits < src_bits {
                    Ok(self
                        .builder
                        .build_int_truncate(v, target_type, "trunc")
                        .map_err(map_err)?
                        .into())
                } else {
                    Ok(v.into())
                }
            }

            (BasicValueEnum::IntValue(v), t) if t.is_float_type() => {
                let target_type = t.into_float_type();
                Ok(self
                    .builder
                    .build_signed_int_to_float(v, target_type, "itof")
                    .map_err(map_err)?
                    .into())
            }

            (BasicValueEnum::FloatValue(v), t) if t.is_int_type() => {
                let target_type = t.into_int_type();
                Ok(self
                    .builder
                    .build_float_to_signed_int(v, target_type, "ftoi")
                    .map_err(map_err)?
                    .into())
            }

            (BasicValueEnum::FloatValue(v), t) if t.is_float_type() => {
                let target_type = t.into_float_type();
                if v.get_type().get_bit_width() < target_type.get_bit_width() {
                    Ok(self
                        .builder
                        .build_float_ext(v, target_type, "fext")
                        .map_err(map_err)?
                        .into())
                } else {
                    Ok(self
                        .builder
                        .build_float_trunc(v, target_type, "ftrunc")
                        .map_err(map_err)?
                        .into())
                }
            }

            (BasicValueEnum::PointerValue(v), t) if t.is_pointer_type() => {
                let target_type = t.into_pointer_type();
                Ok(self
                    .builder
                    .build_bit_cast(v, target_type, "ptrcast")
                    .map_err(map_err)?
                    .into())
            }

            _ => Err(format!("unsupported cast")),
        }
    }
}

fn map_err(e: inkwell::builder::BuilderError) -> String {
    format!("LLVM builder error: {}", e)
}
