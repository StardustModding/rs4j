//! Base methods

use super::{ctx::ClassCtx, field::Field};

/// Generate the `free()` native method code for Java
pub fn free_method_java() -> &'static str {
    "    private static native void jni_free(long ptr);"
}

/// Generate the `free()` wrapper method code for Java
pub fn free_method_java_wrapper() -> &'static str {
    "    public void free() {
        jni_free(__ptr);
    }"
}

/// Generate the `free()` native method code for Rust
pub fn free_method_rust(cls: &ClassCtx, fields: &Vec<Field>) -> String {
    let head = "#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
    missing_docs,
)]";

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
    format!("{head}
pub unsafe extern \"system\" fn Java_{method}<'local, {generics}>(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {{
    let it = Box::from_raw(ptr as *mut {class});
    {frees}
}}")
}

/// Create the `of()` function
pub fn of_func(cx: &ClassCtx, fields: &Vec<Field>) -> String {
    let head = "#[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]";

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

    format!("    {head}\n    pub unsafe fn of(base: {class}) -> Self {{\n        Self {{\n{field_setters}\n        }}\n    }}")
}
