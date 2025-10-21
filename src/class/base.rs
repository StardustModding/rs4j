//! Base methods

use super::{ctx::ClassCtx, field::Field};
use crate::codegen::java::{JCall, JExpr, JExternMethod, JMember, JMethodImpl, JType};
use std::collections::BTreeMap;

pub(crate) const RUST_BRIDGE_HEAD: &str = "#[unsafe(no_mangle)]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs,
    unsafe_op_in_unsafe_fn,
)]";

pub(crate) const RUST_BRIDGE_HEAD_MANGLE: &str = "#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs,
    unsafe_op_in_unsafe_fn,
)]";

/// Generate the `free()` native method code for Java
pub fn free_method_java() -> JMember {
    JMember::ExternMethod(JExternMethod {
        name: "jni_free".into(),
        private: true,
        is_static: true,
        ret: JType::Void,
        args: vec![("ptr".into(), JType::Long)],
    })
}

/// Generate the `free()` wrapper method code for Java
pub fn free_method_java_wrapper() -> JMember {
    JMember::MethodImpl(JMethodImpl {
        name: "free".into(),
        ret: JType::Void,
        is_static: false,
        private: false,
        args: Vec::new(),
        is_override: false,
        generics: BTreeMap::new(),
        code: vec![JExpr::Call(JCall {
            target: "jni_free".into(),
            args: vec![JExpr::Name("__ptr".into())],
        })],
    })
}

/// Generate the `free()` native method code for Rust
pub fn free_method_rust(cls: &ClassCtx, fields: &Vec<Field>) -> String {
    let method = cls.method_name("jni_free");
    let class = &cls.name_generics();
    let mut frees = Vec::new();

    for field in fields {
        if !field.is_primitive() {
            frees.push(format!("let _ = Box::from_raw(it.{});", field.name));
        }
    }

    let frees = frees.join("\n");

    let generics = cls
        .generics
        .iter()
        .map(|v| v.code())
        .collect::<Vec<_>>()
        .join(", ");

    // FIXME: This WILL cause a memory leak if an object is more than two levels deep. FIX THIS!
    format!("{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{method}<'local, {generics}>(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {{
    let it = Box::from_raw(ptr as *mut {class});
    {frees}
}}")
}

/// Create the `of()` function
pub fn of_func(cx: &ClassCtx, fields: &Vec<Field>) -> String {
    let class = &cx.raw_name_generics();
    let mut field_setters = Vec::new();

    for field in fields {
        if field.is_primitive() {
            field_setters.push(format!("            {0}: base.{0}.clone(),", field.name));
        } else {
            field_setters.push(format!(
                "            {0}: Box::leak(Box::new(base.{0})) as *mut {1},",
                field.name,
                field.ty.full_type()
            ));
        }
    }

    if cx.wrapped {
        field_setters.push(format!("            __inner: base,"));
    }

    let field_setters = field_setters.join("\n");

    format!(
        "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn of(base: {class}) -> Self {{\n        Self {{\n{field_setters}\n        }}\n    }}"
    )
}
