//! Native methods.

use super::{ctx::ClassCtx, field::Field, generic::TypeGeneric, method::Method, ty::TypeKind};
use crate::{
    class::{
        base::{RUST_BRIDGE_HEAD, RUST_BRIDGE_HEAD_MANGLE},
        conv::conversion_method,
    },
    codegen::java::{JExternMethod, JMember, JType},
    if_else,
};

impl Method {
    /// Generate Java code for this method.
    pub fn native_java_code(&self) -> JMember {
        let name = if_else!(
            self.is_init,
            format!("jni_init_{}", self.name),
            format!("jni_{}", self.name)
        );

        let ret = self.ret.kind.j_type();
        let mut args = Vec::new();

        if !self.is_static {
            args.push(("ptr".into(), JType::Long));
        }

        for arg in &self.args {
            if arg.ty.kind.is_primitive() {
                args.push((arg.name.clone(), arg.ty.kind.j_type()));
            } else {
                args.push((arg.name.clone(), JType::Long));
            }
        }

        JMember::ExternMethod(JExternMethod {
            name,
            private: true,
            ret: if_else!(self.is_init, JType::Long, ret),
            is_static: !self.is_init,
            args,
        })
    }

    /// Generate Rust code for this method.
    pub fn native_rust_code(
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
            .map(|v| v.code())
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
            let j = if_else!(
                cx.generics
                    .iter()
                    .find(|v| v.name == self.ret.kind.rust_name())
                    .is_none(),
                "__JNI_",
                ""
            );

            if self.is_consumed {
                cpost.push_str("let val = ");
                cpost2.push_str(&format!(
                    ";\n    (Box::leak(Box::new(val)) as *mut {j}{rt}) as jlong"
                ));
            } else {
                post.push_str("let val = ");
                post2.push_str(&format!(
                    ";\n    (Box::leak(Box::new(val)) as *mut {j}{rt}) as jlong"
                ));
            }
        }

        if self.is_init {
            if self.is_optional {
                format!(
                    "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}
    let it = {class}::__wrapped_{method}({args_nt});

    if let Some(it) = it {{
        (Box::leak(Box::new(it)) as *mut {class_c}) as {ret}
    }} else {{
        JObject::null().as_raw() as {ret}
    }}
}}"
                )
            } else {
                format!(
                    "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}
    let it = {class}::__wrapped_{method}({args_nt});
    (Box::leak(Box::new(it)) as *mut {class_c}) as {ret}
}}"
                )
            }
        } else {
            if self.is_static {
                if self.is_optional {
                    format!(
                        "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    let val = {class}::__wrapped_{method}({args_nt});

    if let Some(val) = val {{
        {post}val{post2}
    }} else {{
        JObject::null().as_raw() as {ret}
    }}
}}"
                    )
                } else {
                    format!(
                        "{RUST_BRIDGE_HEAD}
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
                            "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    let val = {post}it.__wrapped_{method}({args_nt}).unwrap_or_default(){post2};
    let it = Box::from_raw(ptr as *mut {class_c});

    {frees}

    if let Some(val) = val {{
        {cpost}val{cpost2}
    }} else {{
        JObject::null().as_raw() as {ret}
    }}
}}"
                        )
                    } else {
                        format!(
                            "{RUST_BRIDGE_HEAD}
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
                            "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>({base_args}, {args}) -> {ret} {{
    {pre}

    let val = it.__wrapped_{method}({args_nt});

    if let Some(val) = val {{
        {post}val{post2}
    }} else {{
        JObject::null().as_raw() as {ret}
    }}
}}"
                        )
                    } else {
                        format!(
                            "{RUST_BRIDGE_HEAD}
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
    pub fn native_rust_wrapper_code(&self, cx: &ClassCtx) -> String {
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

        let mut pre = if_else!(self.boxed, "Box::new(", "").to_string();
        let mut post = if_else!(self.boxed, ")", "").to_string();

        if !self.ret.kind.is_primitive()
            && cx
                .generics
                .iter()
                .find(|v| v.name == self.ret.kind.rust_name())
                .is_none()
        {
            ret = format!("__JNI_{}", ret);

            pre.push_str(&format!("__JNI_{}::of(", self.ret.kind.rust_name()));
            post.push_str(")");
        }

        if self.is_optional {
            ret = format!("Option<{}>", ret);
        }

        if self.is_static {
            if self.is_init {
                if self.is_optional {
                    format!(
                        "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}({args}) -> Option<Self> {{\n        let base = {tclass}::{tmethod}({args_nt});\n\n        if let Some(base) = base {{\n            Some(Self::of(base))\n        }} else {{\n            None\n        }}\n    }}"
                    )
                } else {
                    format!(
                        "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}({args}) -> Self {{\n        let base = {tclass}::{tmethod}({args_nt});\n\n        Self::of(base)\n    }}"
                    )
                }
            } else {
                if self.is_optional {
                    format!(
                        "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}({args}) -> {ret} {{\n        let val = {tclass}::{tmethod}({args_nt});\n        if let Some(val) = val {{\n            Some({pre}val{post})\n        }} else {{\n            None\n        }}\n    }}"
                    )
                } else {
                    format!(
                        "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}({args}) -> {ret} {{\n        {pre}{tclass}::{tmethod}({args_nt}){post}\n    }}"
                    )
                }
            }
        } else {
            if self.is_optional {
                format!(
                    "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}(&{m_mut}self, {args}) -> {ret} {{\n        let val = {tclass}::{tmethod}({args_nt});\n        if let Some(val) = val {{\n            Some({pre}val.clone(){post})\n        }} else {{\n            None\n        }}\n    }}"
                )
            } else {
                format!(
                    "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn __wrapped_{method}(&{m_mut}self, {args}) -> {ret} {{\n        {pre}{tclass}::{tmethod}({args_nt}).clone(){post}\n    }}"
                )
            }
        }
    }
}
