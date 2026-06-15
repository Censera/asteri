use crate::ast::{BinaryOp, Expr, Stmt, Types};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, IntValue, PointerValue, ValueKind,
};
use inkwell::AddressSpace;
use inkwell::IntPredicate;

use std::collections::HashMap;

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    scopes: Vec<HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>>,
    functions: HashMap<String, (FunctionType<'ctx>, FunctionValue<'ctx>)>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let scopes = vec![HashMap::new()];
        let functions = HashMap::new();

        Self {
            context,
            module,
            builder,
            scopes,
            functions,
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        let printf_tp = self.context.i32_type().fn_type(
            &[self.context.ptr_type(AddressSpace::default()).into()],
            true,
        );

        self.module.add_function("printf", printf_tp, None);

        self.declare_fn(stmts);

        for stmt in stmts {
            self.cmpl_stmt(stmt)?
        }
        Ok(())
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
                    self.define_v(name.clone(), (alc, v.get_type()));
                }
                Ok(())
            }

            Stmt::Expr { expr, .. } => {
                self.cmpl_expr(expr)?;
                Ok(())
            }

            Stmt::Print(expr) => {
                let (fmt_v, args) = self.prepare_fmt_and_args(expr)?;
                let printf = self.get_or_create_printf();
                let mut all_args = vec![fmt_v.into()];
                all_args.extend(args.into_iter().map(|v| BasicMetadataValueEnum::from(v)));
                self.builder
                    .build_call(printf, &all_args, "printf")
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

            Stmt::Block(stmts) => {
                self.push_scope();
                for stmt in stmts {
                    self.cmpl_stmt(stmt)?;
                }
                self.pop_scope();
                Ok(())
            }

            Stmt::Fun {
                name,
                rt_tp: _,
                params,
                body,
                ..
            } => {
                let function = self
                    .functions
                    .get(name)
                    .ok_or_else(|| format!("function '{}' is not declared", name))?
                    .1;

                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

                self.push_scope();

                for (i, (p_name, _)) in params.iter().enumerate() {
                    let param = function.get_nth_param(i as u32).unwrap();
                    let alc = self
                        .builder
                        .build_alloca(param.get_type(), p_name)
                        .map_err(map_err)?;
                    self.builder.build_store(alc, param).map_err(map_err)?;
                    self.define_v(p_name.clone(), (alc, param.get_type()));
                }

                for s in body {
                    self.cmpl_stmt(s)?;
                }

                if let Some(last_block) = function.get_last_basic_block() {
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

                self.pop_scope();
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
                let len = s.len() as i64;
                let global_str = self.builder.build_global_string_ptr(s, ".str")
                .map_err(map_err)?;

                let data_ptr = global_str.as_pointer_value();

                let str_tp = self.llvm_tp(&Types::Str).into_struct_type();
                let mut fields = Vec::new();
                fields.push(BasicTypeEnum::PointerValue(data_ptr));
                fields.push(BasicTypeEnum::IntValue(
                    self.context.i64_type().const_int(len, false),
                ));
                let str_v = str_tp.const_named_sturct(&fields);
                Ok(str_v.into())
            }

            Expr::Id { name, .. } => {
                let (ptr, tp) = self
                    .lookup_v(name)
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

            Expr::Call { name, args, .. } => {
                let function = self
                    .functions
                    .get(name)
                    .ok_or_else(|| format!("undefined function '{}'", name))?
                    .1;

                let mut arg_v: Vec<BasicValueEnum> = Vec::with_capacity(args.len());
                for arg in args {
                    arg_v.push(self.cmpl_expr(arg)?);
                }
                let arg_meta: Vec<BasicMetadataValueEnum> =
                    arg_v.iter().map(|&v| v.into()).collect();

                let call_site = self
                    .builder
                    .build_call(function, &arg_meta, "call")
                    .map_err(map_err)?;

                match call_site.try_as_basic_value() {
                    ValueKind::Basic(v) => Ok(v),
                    ValueKind::Instruction(_) => {
                        let dum = self.context.i32_type().const_int(0, false);
                        Ok(BasicValueEnum::IntValue(dum))
                    }
                }
            }

            Expr::FmtStr { raw, .. } => {
                let (fmt_ptr, args) = self.compile_fmt_str(raw, false)?;
                let asprintf = self.get_or_create_asprintf();

                let result_ptr = self
                    .builder
                    .build_alloca(self.context.ptr_type(AddressSpace::default()), "fmt_result")
                    .map_err(map_err)?;

                let mut all_args = vec![result_ptr.into(), fmt_ptr.into()];
                all_args.extend(args.into_iter().map(BasicMetadataValueEnum::from));

                self.builder
                    .build_call(asprintf, &all_args, "asprintf")
                    .map_err(map_err)?;

                let data_ptr = self.builder.build_load(
                    self.context.ptr_type(AddressSpace::default()),
                    result_ptr,
                    "loaded_fmt",
                ).map_err(map_err)?;


                let strlen = self.get_or_create_strlen();
                let let_v = self.builder.build_call(str, &[data_ptr.into()], "strlen").map_err(map_err)?.try_as_basic_value().left().unwrap().into_int_value();

                let str_tp = self.llvm_tp(&Type::Str).into_struct_type();
                let str_v = str_type.const_named_sturct(&[data_ptr.into(), len_v.into()]);

                Ok(str_v.into())
            }

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
            
            Types::Str => {
                let fields = [
                    self.context.ptr_type(AddressSpace::default()).into(), // data
                    self.context.i64_type().into(),  // len
                ];
                self.context.struct_type(&fields, false).into()
            }

            Types::String => {
                let fields = [
                    self.context.ptr_type(AddressSpace::default()).into(), // data
                    self.context.i64_type().into(),  // cap
                    self.context.i64_type().into(), // len
                ];
                self.context.struct_type(&fields, false).into()
            }
            Types::Cstr => {
                self.context.ptr_type(AddressSpace::default()).into
            }
            _ => unimplemented!("type {:?}", tp),
        }
    }

    // ------------------------------------------------------------------------

    fn prepare_fmt_and_args(
        &mut self,
        expr: &Expr,
    ) -> Result<(PointerValue<'ctx>, Vec<BasicValueEnum<'ctx>>), String> {
        match expr {
            Expr::FmtStr { raw, .. } => self.compile_fmt_str(raw, false),

            _ => {
                let (fmt_lit, args) = self.format_single_expr(expr)?;
                let fmt_ptr = self
                    .builder
                    .build_global_string_ptr(&fmt_lit, "fmt")
                    .map_err(map_err)?
                    .as_pointer_value();
                Ok((fmt_ptr, args))
            }
        }
    }

    fn format_single_expr(
        &mut self,
        expr: &Expr,
    ) -> Result<(String, Vec<BasicValueEnum<'ctx>>), String> {
        let v = self.cmpl_expr(expr)?;
        let fmt = match v {
            BasicValueEnum::IntValue(v) => {
                let width = v.get_type().get_bit_width();
                if width == 1 {
                    "%s\n".to_string()
                } else {
                    "%d\n".to_string()
                }
            }

            BasicValueEnum::FloatValue(_) => "%g\n".to_string(),
            BasicValueEnum::PointerValue(_) => {
                // TODO: use type annotation from sema.
                "%s\n".to_string()
            }

            _ => return Err("unsupported type for print".into()),
        };

        let formatted = self.convert_for_printf(v)?;
        Ok((fmt, vec![formatted]))
    }

    fn compile_fmt_str(
        &mut self,
        raw: &str,
        newline: bool,
    ) -> Result<(PointerValue<'ctx>, Vec<BasicValueEnum<'ctx>>), String> {
        let bind = Self::unescape_string(raw);
        let raw = bind.as_str();
        let mut fmt_lit = String::new();
        let mut args = Vec::new();
        let mut rem = raw;

        while let Some(start) = rem.find('{') {
            let end = rem[start..]
                .find('}')
                .ok_or_else(|| "unclosed '{' in format string".to_string())?;
            fmt_lit.push_str(&rem[..start]);
            let ident = &rem[start + 1..start + end];
            let v = self.cmpl_expr(&Expr::Id {
                name: ident.to_string(),
                line: 0,
            })?;

            let spec = match v {
                BasicValueEnum::IntValue(_) => "%d",
                BasicValueEnum::FloatValue(_) => "%g",
                BasicValueEnum::PointerValue(_) => "%s",
                _ => return Err("unsupported type in format string".into()),
            };

            fmt_lit.push_str(spec);
            args.push(self.convert_for_printf(v)?);

            rem = &rem[start + end + 1..];
        }

        fmt_lit.push_str(rem);
        if newline {
            fmt_lit.push('\n');
        }

        let fmt_ptr = self
            .builder
            .build_global_string_ptr(&fmt_lit, "fmt")
            .map_err(map_err)?
            .as_pointer_value();

        Ok((fmt_ptr, args))
    }

    fn convert_for_printf(&self, v: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        match v {
            BasicValueEnum::IntValue(v) if v.get_type().get_bit_width() < 32 => {
                let ext = self
                    .builder
                    .build_int_s_extend_or_bit_cast(v, self.context.i32_type(), "ext")
                    .map_err(map_err)?;
                Ok(ext.into())
            }
            BasicValueEnum::FloatValue(v) if v.get_type().get_bit_width() < 64 => {
                let ext = self
                    .builder
                    .build_float_ext(v, self.context.f64_type(), "fext")
                    .map_err(map_err)?;
                Ok(ext.into())
            }
            BasicValueEnum::StructValue(_sv) => {
                // TODO: AST type to know the layout.
                Err("Direct printing of structs not yet supported".into())
            }

            other => Ok(other),
        }
    }

    fn get_or_create_printf(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("printf") {
            return func;
        }
        let printf_tp = self.context.i32_type().fn_type(
            &[self.context.ptr_type(AddressSpace::default()).into()],
            true,
        );
        self.module.add_function("printf", printf_tp, None)
    }

    fn get_or_create_asprintf(&self) -> FunctionValue<'ctx> {
        if let Some(function) = self.module.get_function("asprintf") {
            return function;
        };

        let asprintf_tp = self.context.i32_type().fn_type(
            &[
                self.context.ptr_type(AddressSpace::default()).into(),
                self.context.ptr_type(AddressSpace::default()).into(),
            ],
            true,
        );
        self.module.add_function("asprintf", asprintf_tp, None)
    }

    fn declare_fn(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Fun {
                    name,
                    rt_tp,
                    params,
                    ..
                } => {
                    let fn_tp = self.fn_tp(name, rt_tp, params);
                    let function = self.module.add_function(name, fn_tp, None);
                    self.functions.insert(name.clone(), (fn_tp, function));
                }

                Stmt::Block(stmts) => self.declare_fn(stmts),

                _ => {}
            }
        }
    }

    fn  get_or_create_strlen(&self) -> FunctionValue {
        if let Some(function) = self.module.get_function("strlen") {
            return function;
        } else {
            let strlen_tp = self.context.i64_type().fn_type(
                &[self.context.ptr_type(AddressSpace::default()).into()],
                false,
            );
            self.module.add_function("strlen", strlen_tp, None)
        }
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

    // ------------------------------------------------------------------------

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop().expect("scope stack underflow");
    }

    fn define_v(&mut self, name: String, variable: (PointerValue<'ctx>, BasicTypeEnum<'ctx>)) {
        self.scopes
            .last_mut()
            .expect("no active scope")
            .insert(name, variable);
    }

    fn lookup_v(&self, name: &str) -> Option<&(PointerValue<'ctx>, BasicTypeEnum<'ctx>)> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None
    }

    fn unescape_string(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('{') => out.push('{'),
                    Some('}') => out.push('}'),
                    Some(c) => {
                        out.push('\\');
                        out.push(c);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}

fn map_err(e: inkwell::builder::BuilderError) -> String {
    format!("LLVM builder error: {}", e)
}
