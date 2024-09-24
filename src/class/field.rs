//! Fields.

use super::{ctx::ClassCtx, ty::{Type, TypeKind}};

/// A field in a class.
pub struct Field {
    /// The name of this field.
    pub name: String,

    /// This field's [`Type`].
    pub ty: Type,
}

impl Field {
    /// Create a new [`Field`].
    pub fn new(name: impl AsRef<str>, ty: Type) -> Self {
        Self {
            name: name.as_ref().into(),
            ty,
        }
    }

    /// Generate Java code for a setter.
    pub fn java_setter(&self) -> String {
        let name = format!("jni_set_{}", &self.name);
        let ty = self.ty.full_type_java();

        format!("    private static native long {name}(long ptr, {ty});\n")
    }

    /// Generate Rust code for a setter.
    pub fn rust_setter(&self, cx: &ClassCtx) -> String {
        let name = cx.method_name(format!("jni_set_{}", &self.name));
        let class = cx.name();
        let field = &self.name;

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

        if self.ty.kind.is_number() {
            let val_ty = self.ty.kind.java_ty();

            format!("{head}
pub unsafe extern \"system\" fn {name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: {val_ty},
) -> jlong {{
    let it = &mut *(ptr as *mut {class});

    it.{field} = val;

    ptr as jlong
}}")
        } else if self.ty.kind == TypeKind::String {
            format!("{head}
pub unsafe extern \"system\" fn {name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: JString<'local>,
) -> jlong {{
    let it = &mut *(ptr as *mut {class});
    let val = env.get_string(&val).unwrap().to_str().unwrap().to_string();

    it.{field} = val;

    ptr as jlong
}}")
        } else {
            let other_name = self.ty.full_type();

            format!("{head}
pub unsafe extern \"system\" fn {name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jlong,
) -> jlong {{
    let it = &mut *(ptr as *mut {class});

    it.{field} = val as *mut {other_name};

    ptr as jlong
}}")
        }
    }
}
