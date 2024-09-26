//! Native methods.

use super::{
    arg::FunctionArg,
    ctx::ClassCtx,
    field::Field,
    generic::TypeGeneric,
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

    /// Does this return an [`Option`]?
    pub is_optional: bool,

    /// Does it consume the object?
    pub is_consumed: bool,

    /// Is there another struct this should use to call the function?
    pub object: Option<String>,

    /// Is there a custom name?
    pub custom_name: Option<String>,

    /// Does it need to be boxed?
    pub boxed: bool,
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
    pub fn rust_code(
        &self,
        cx: &ClassCtx,
        fields: &Vec<Field>,
        generics_list: &Vec<TypeGeneric>,
    ) -> String {
        let class = cx.name();
        let class_c = cx.name_generics();
        let method = &self.name;

        let name = cx.method_name(if_else!(
            self.is_init,
            format!("jni_init_{}", self.name),
            format!("jni_{}", self.name)
        ));

        let generics = generics_list
            .iter()
            .map(|v| format!("{}: {}", v.name, v.bounds.join(" + ")))
            .collect::<Vec<_>>()
            .join(", ");

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
                    TypeKind::U8 => args_nt.push(format!("{} as u8", arg.name.clone())),
                    TypeKind::U16 => args_nt.push(format!("{} as u16", arg.name.clone())),
                    TypeKind::U32 => args_nt.push(format!("{} as u32", arg.name.clone())),
                    TypeKind::U64 => args_nt.push(format!("{} as u64", arg.name.clone())),
                    TypeKind::Bool => args_nt.push(format!("{} == 1", arg.name.clone())),
                    _ => args_nt.push(arg.name.clone()),
                }
            }
        }

        let args = args.join(", ");
        let args_nt = args_nt.join(", ");
        let mut_ = if_else!(self.is_mut, "mut ", "");

        let mut conversions = Vec::new();

        if !self.is_static {
            conversions.push(format!("let it = &{mut_}*(ptr as *mut {class_c});"));
        }

        for arg in &self.args {
            if let Some(conv) = conversion_method(&arg.name, &arg.ty, arg.mutable) {
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

        let mut post =
            if_else!(self.ret.kind == TypeKind::String, "env.new_string(", "").to_string();
        let mut post2 =
            if_else!(self.ret.kind == TypeKind::String, ").unwrap().as_raw()", "").to_string();

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

        match self.ret.kind {
            TypeKind::U8 => post2.push_str(" as i8"),
            TypeKind::U16 => post2.push_str(" as i16"),
            TypeKind::U32 => post2.push_str(" as i32"),
            TypeKind::U64 => post2.push_str(" as i64"),
            TypeKind::Bool => post2.push_str(" as u8"),

            _ => {}
        }

        let mut cpost = "".to_string();
        let mut cpost2 = "".to_string();

        if !self.ret.kind.is_primitive() {
            let rt = self.ret.full_type();

            if self.is_consumed {
                cpost.push_str("let val = ");
                cpost2.push_str(&format!(
                    ";\n    (Box::leak(Box::new(val)) as *mut {rt}) as jlong"
                ));
            } else {
                post.push_str("let val = ");
                post2.push_str(&format!(
                    ";\n    (Box::leak(Box::new(val)) as *mut {rt}) as jlong"
                ));
            }
        }

        if self.is_init {
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}
    let it = {class}::__wrapped_{method}({args_nt});
    (Box::leak(Box::new(it)) as *mut {class_c}) as jlong
}}"
            )
        } else {
            if self.is_static {
                if self.is_optional {
                    format!(
                        "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}{class}::__wrapped_{method}({args_nt}).unwrap_or_default(){post2}
}}"
                    )
                } else {
                    format!(
                        "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}{class}::__wrapped_{method}({args_nt}){post2}
}}"
                    )
                }
            } else {
                if self.is_consumed {
                    let mut frees = Vec::new();

                    for field in fields {
                        if !field.is_primitive() {
                            frees.push(format!("let _ = Box::from_raw(it.{});", field.name));
                        }
                    }

                    let frees = frees.join("\n");

                    if self.is_optional {
                        format!(
                            "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    let val = {post}it.__wrapped_{method}({args_nt}).unwrap_or_default(){post2};
    let it = Box::from_raw(ptr as *mut {class_c});
    {frees}

    {cpost}val{cpost2}
}}"
                        )
                    } else {
                        format!(
                            "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    let val = {post}it.__wrapped_{method}({args_nt}){post2};
    let it = Box::from_raw(ptr as *mut {class_c});
    {frees}

    {cpost}val{cpost2}
}}"
                        )
                    }
                } else {
                    if self.is_optional {
                        format!(
                            "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}it.__wrapped_{method}({args_nt}).unwrap_or_default(){post2}
}}"
                        )
                    } else {
                        format!(
                            "{head}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    {post}it.__wrapped_{method}({args_nt}){post2}
}}"
                        )
                    }
                }
            }
        }
    }

    /// Generate the impl for the wrapper struct.
    pub fn rust_code_wrapper(&self, cx: &ClassCtx, fields: &Vec<Field>) -> String {
        let class = &cx.name;
        let method = &self.name;
        let tclass = self.object.clone().unwrap_or(class.clone());
        let tmethod = self.custom_name.clone().unwrap_or(method.clone());
        let mut args = Vec::new();
        let mut args_nt = Vec::new();
        let m_mut = if_else!(self.is_mut, "mut ", "");

        if !self.is_static {
            if self.is_consumed {
                args_nt.push("self.to_rust()".into());
            } else {
                if self.is_mut {
                    args_nt.push("&mut self.to_rust()".into());
                } else {
                    args_nt.push("&self.to_rust()".into());
                }
            }
        }

        for arg in &self.args {
            let borrow = if_else!(arg.borrow, "&", "");
            let mut_ = if_else!(arg.mutable, "mut ", "");

            args.push(format!(
                "{}: {borrow}{mut_}{}",
                arg.name,
                arg.ty.full_type()
            ));

            args_nt.push(arg.name.clone());
        }

        let mut ret = self.ret.full_type();
        let args = args.join(", ");
        let args_nt = args_nt.join(", ");

        if self.is_optional {
            ret = format!("Option<{}>", ret);
        }

        let pre = if_else!(self.boxed, "Box::new(", "");
        let post = if_else!(self.boxed, ")", "");

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

                format!("    pub unsafe fn __wrapped_{method}({args}) -> Self {{\n        let base = {tclass}::{tmethod}({args_nt});\n\n        Self {{\n{field_setters}\n        }}\n    }}")
            } else {
                format!("    pub unsafe fn __wrapped_{method}({args}) -> {ret} {{\n        {pre}{tclass}::{tmethod}({args_nt}){post}\n    }}")
            }
        } else {
            format!("    pub unsafe fn __wrapped_{method}(&{m_mut}self, {args}) -> {ret} {{\n        {pre}{tclass}::{tmethod}({args_nt}).clone(){post}\n    }}")
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
            is_consumed: value.is_consumed,
            is_optional: value.is_optional,
            ret: value.ret.map(|v| v.into()).unwrap_or_default(),
            custom_name: value.rust_name.map(|v| v.ident_strict().unwrap()),
            object: value.source.map(|v| v.ident_strict().unwrap()),
            boxed: value.boxed,
        }
    }
}
