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
    let method = cls.method_name("jni_free");
    let class = &cls.name();
    let mut frees = Vec::new();

    for field in fields {
        if !field.is_primitive() {
            frees.push(format!("let _ = Box::from_raw(it.{});", field.name));
        }
    }

    let frees = frees.join("\n");

    // TODO: This WILL cause a memory leak if an object is more than two levels deep. FIX THIS!
    format!("#[no_mangle]
#[allow(unused)]
pub unsafe extern \"system\" fn Java_{method}<'local>(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {{
    let it = Box::from_raw(ptr as *mut {class});
    {frees}
}}")
}