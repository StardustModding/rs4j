//! Fields.

use convert_case::{Case, Casing};

use crate::parser::field::FieldExpr;

use super::{
    ctx::ClassCtx,
    ty::{Type, TypeKind},
};

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

    /// Is the type primitive?
    pub fn is_primitive(&self) -> bool {
        self.ty.kind.is_primitive()
    }

    /// Generate Java code for a setter.
    pub fn java_setter(&self) -> String {
        let name = format!("jni_set_{}", &self.name);
        let ty = self.ty.full_type_java();

        format!("    private static native long {name}(long ptr, {ty} value);\n")
    }

    /// Generate Java wrapper code for a setter.
    pub fn java_setter_wrapper(&self) -> String {
        let native = format!("jni_set_{}", &self.name);
        let name = format!("set_{}", &self.name).to_case(Case::Camel);
        let ty = self.ty.full_type_java();

        format!("    public static void {name}({ty} value) {{\n        {native}(__ptr, value);\n        if (__parent != null) {{\n            __parent.updateField(__parentField, __ptr);\n        }}\n    }}")
    }

    /// Generate Java code for a getter.
    pub fn java_getter(&self) -> String {
        let name = format!("jni_get_{}", &self.name);
        let ty = self.ty.full_type_java();

        format!("    private static native {ty} {name}(long ptr);\n")
    }

    /// Generate Java wrapper code for a getter.
    pub fn java_getter_wrapper(&self) -> String {
        let field = &self.name;
        let native = format!("jni_get_{}", &self.name);
        let name = format!("get_{}", &self.name).to_case(Case::Camel);
        let ty = self.ty.full_type_java();

        if self.is_primitive() {
            format!("    public static {ty} {name}() {{\n        return {native}(__ptr);\n    }}")
        } else {
            format!("    public static {ty} {name}() {{\n        return {ty}.from({native}(__ptr), this, {field});\n    }}")
        }
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
            let val_ty = self.ty.kind.jni_name();

            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: {val_ty},
) -> jlong {{
    let it = &mut *(ptr as *mut {class});

    it.{field} = val;

    ptr as jlong
}}"
            )
        } else if self.ty.kind == TypeKind::String {
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: JString<'local>,
) -> jlong {{
    let it = &mut *(ptr as *mut {class});
    let val = env.get_string(&val).unwrap().to_str().unwrap().to_string();

    it.{field} = val;

    ptr as jlong
}}"
            )
        } else {
            let other_name = self.ty.full_type();

            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    val: jlong,
) -> jlong {{
    let it = &mut *(ptr as *mut {class});

    it.{field} = val as *mut {other_name};

    ptr as jlong
}}"
            )
        }
    }

    /// Generate Rust code for a getter.
    pub fn rust_getter(&self, cx: &ClassCtx) -> String {
        let name = cx.method_name(format!("jni_get_{}", &self.name));
        let class = cx.name();
        let field = &self.name;
        let ret = self.ty.kind.jni_name();

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
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> {ret} {{
    let it = &*(ptr as *mut {class});

    it.{field} as {ret}
}}"
            )
        } else if self.ty.kind == TypeKind::String {
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jstring {{
    let it = &*(ptr as *mut {class});
    env.new_string(it.{field}.clone()).unwrap().as_raw()
}}"
            )
        } else {
            format!(
                "{head}
pub unsafe extern \"system\" fn Java_{name}<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jlong {{
    let it = &mut *(ptr as *mut {class});

    it.{field} as jlong
}}"
            )
        }
    }
}

impl From<FieldExpr> for Field {
    fn from(value: FieldExpr) -> Self {
        Self {
            name: value.name.ident_strict().unwrap(),
            ty: value.ty.into(),
        }
    }
}
