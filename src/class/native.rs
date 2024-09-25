//! Native methods.

use super::{
    arg::FunctionArg,
    ctx::ClassCtx,
    field::Field,
    ty::{Type, TypeKind},
};
use crate::{class::conv::conversion_method, if_else, parser::func::FunctionExpr};

/// A native method.
pub struct NativeMethod {
    /// The name of this method.
    pub name: String,

    /// This method's args.
    pub args: Vec<FunctionArg>,

    /// The return type.
    pub ret: Type,

    /// Is this method static?
    pub is_static: bool,

    /// Does it mutate the object?
    pub is_mut: bool,

    /// Is this a constructor?
    pub is_init: bool,
}

impl NativeMethod {
    /// Generate Java code for this method.
    pub fn java_code(&self) -> String {
        let name = if_else!(
            self.is_init,
            format!("jni_init_{}", self.name),
            format!("jni_{}", self.name)
        );

        let ret = self.ret.kind.native_name();
        let mut args = Vec::new();

        if !self.is_static {
            args.push("long ptr".into());
        }

        for arg in &self.args {
            args.push(format!("{} {}", arg.ty.kind.native_name(), arg.name));
        }

        let args = args.join(", ");

        if self.is_init {
            format!("    private native long {name}({args});")
        } else {
            format!("    private static native {ret} {name}({args});")
        }
    }

    /// Generate Rust code for this method.
    pub fn rust_code(&self, cx: &ClassCtx) -> String {
        let class = cx.name();
        let method = &self.name;
        let name = cx.method_name(if_else!(
            self.is_init,
            format!("jni_init_{}", self.name),
            format!("jni_{}", self.name)
        ));
        let mut args = Vec::new();
        let mut args_nt = Vec::new();

        for arg in &self.args {
            args.push(format!("{}: {}", arg.name, arg.ty.kind.jni_arg_name()));

            if arg.borrow && !arg.ty.kind.is_primitive() {
                if arg.mutable {
                    args_nt.push(format!("&mut {}", arg.name.clone()));
                } else {
                    args_nt.push(format!("&{}", arg.name.clone()));
                }
            } else {
                match arg.ty.kind {
                    TypeKind::String | TypeKind::Other(_) => {
                        args_nt.push(format!("{}.clone()", arg.name.clone()))
                    }
                    _ => args_nt.push(arg.name.clone()),
                }
            }
        }

        let args = args.join(", ");
        let args_nt = args_nt.join(", ");
        let mut_ = if_else!(self.is_mut, "mut ", "");

        let mut conversions = Vec::new();

        if !self.is_static {
            conversions.push(format!("let it = &{mut_}*(ptr as *mut {class});"));
        }

        for arg in &self.args {
            if let Some(conv) = conversion_method(&arg.name, &arg.ty.kind, arg.mutable) {
                conversions.push(conv);
            }
        }

        // Native methods are ALWAYS static
        let base_args = if self.is_init {
            "mut env: JNIEnv<'local>, obj: JObject<'local>"
        } else {
            "mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong"
        };

        let ret = self.ret.kind.jni_name();

        let post = if_else!(self.ret.kind == TypeKind::String, "env.new_string(", "");
        let post2 = if_else!(self.ret.kind == TypeKind::String, ").unwrap().as_raw()", "");

        let head = "#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs
)]";

        let pre = conversions.join("\n");

        if self.is_init {
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>({base_args}, {args}) -> {ret} {{
    let it = {class}::__wrapped_{method}({args_nt});
    (Box::leak(Box::new(it)) as *mut {class}) as jlong
}}"
            )
        } else {
            if self.is_static {
                format!(
                    "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}{class}::__wrapped_{method}({args_nt}){post2}
}}"
                )
            } else {
                format!(
                    "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}it.__wrapped_{method}({args_nt}){post2}
}}"
                )
            }
        }
    }

    /// Generate the impl for the wrapper struct.
    pub fn rust_code_wrapper(&self, cx: &ClassCtx, fields: &Vec<Field>) -> String {
        let class = &cx.name;
        let method = &self.name;
        let mut args = Vec::new();
        let mut args_nt = Vec::new();
        let m_mut = if_else!(self.is_mut, "mut ", "");

        for arg in &self.args {
            let borrow = if_else!(arg.borrow, "&", "");
            let mut_ = if_else!(arg.mutable, "mut ", "");

            args.push(format!(
                "{}: {borrow}{mut_}{}",
                arg.name,
                arg.ty.kind.rust_name()
            ));

            args_nt.push(arg.name.clone());
        }

        let ret = self.ret.full_type();
        let args = args.join(", ");
        let args_nt = args_nt.join(", ");

        if self.is_static {
            if self.is_init {
                let mut field_setters = Vec::new();

                for field in fields {
                    if field.is_primitive() {
                        field_setters
                            .push(format!("            {0}: base.{0}.clone(),", field.name));
                    } else {
                        field_setters.push(format!(
                            "            {0}: Box::leak(Box::new(base.{0})) as *mut {1},",
                            field.name,
                            field.ty.full_type()
                        ));
                    }
                }

                let field_setters = field_setters.join("\n");

                format!("    pub unsafe fn __wrapped_{method}({args}) -> Self {{\n        let base = {class}::{method}({args_nt});\n\n        Self {{\n{field_setters}\n        }}\n    }}")
            } else {
                format!("    pub unsafe fn __wrapped_{method}({args}) -> {ret} {{\n        {class}::{method}({args_nt})\n    }}")
            }
        } else {
            format!("    pub unsafe fn __wrapped_{method}(&{m_mut}self, {args}) -> {ret} {{\n        self.to_rust().{method}({args_nt})\n    }}")
        }
    }
}

impl From<FunctionExpr> for NativeMethod {
    fn from(value: FunctionExpr) -> Self {
        Self {
            name: value.name.ident_strict().unwrap(),
            args: value
                .args
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect::<Vec<_>>(),
            is_init: value.is_init,
            is_mut: value.is_mut,
            is_static: value.is_static,
            ret: value.ret.map(|v| v.into()).unwrap_or_default(),
        }
    }
}
