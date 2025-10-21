//! Fields.

use convert_case::{Case, Casing};
use std::collections::BTreeMap;

use crate::{
    class::base::RUST_BRIDGE_HEAD,
    codegen::java::{
        JCall, JExpr, JExternMethod, JGetterImpl, JMember, JMethodImpl, JSafeFieldCall, JSetField,
        JType,
    },
    if_else,
};

use super::{
    ctx::ClassCtx,
    ty::{Type, TypeKind},
};

/// A field in a class.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    /// The name of this field.
    pub name: String,

    /// This field's [`Type`].
    pub ty: Type,

    /// Is this field rust-only?
    pub rust: bool,
}

impl Field {
    /// Create a new [`Field`].
    pub fn new(name: impl AsRef<str>, ty: Type) -> Self {
        Self {
            name: name.as_ref().into(),
            ty,
            rust: false,
        }
    }

    /// Is the type primitive?
    pub fn is_primitive(&self) -> bool {
        self.ty.kind.is_primitive()
    }

    /// Generate Java code for a setter.
    pub fn java_setter(&self) -> JMember {
        let name = format!("jni_set_{}", &self.name);

        JMember::ExternMethod(JExternMethod {
            name,
            is_static: true,
            private: true,
            ret: JType::Long,
            args: vec![
                ("ptr".into(), JType::Long),
                (
                    "value".into(),
                    if_else!(self.is_primitive(), self.ty.j_type(), JType::Long),
                ),
            ],
        })
    }

    /// Generate Java wrapper code for a setter.
    pub fn java_setter_wrapper(&self) -> JMember {
        let native = format!("jni_set_{}", &self.name);
        let name = format!("set_{}", &self.name).to_case(Case::Camel);

        if self.is_primitive() {
            JMember::MethodImpl(JMethodImpl {
                name,
                private: false,
                is_override: false,
                is_static: false,
                ret: JType::Void,
                args: vec![("value".into(), self.ty.j_type())],
                generics: BTreeMap::new(),
                code: vec![
                    JExpr::SetField(JSetField {
                        target: "__ptr".into(),
                        value: Box::new(JExpr::Call(JCall {
                            target: native,
                            args: vec![JExpr::Name("__ptr".into()), JExpr::Name("value".into())],
                        })),
                    }),
                    JExpr::SafeFieldCall(JSafeFieldCall {
                        field: "__parent".into(),
                        target: "updateField".into(),
                        args: vec![
                            JExpr::Name("__parentField".into()),
                            JExpr::Name("__ptr".into()),
                        ],
                    }),
                ],
            })
        } else {
            JMember::MethodImpl(JMethodImpl {
                name,
                private: false,
                is_override: false,
                is_static: false,
                ret: JType::Void,
                args: vec![("value".into(), self.ty.j_type())],
                generics: BTreeMap::new(),
                code: vec![
                    JExpr::SetField(JSetField {
                        target: "__ptr".into(),
                        value: Box::new(JExpr::Call(JCall {
                            target: native,
                            args: vec![
                                JExpr::Name("__ptr".into()),
                                JExpr::GetPointer("value".into()),
                            ],
                        })),
                    }),
                    JExpr::SafeFieldCall(JSafeFieldCall {
                        field: "__parent".into(),
                        target: "updateField".into(),
                        args: vec![
                            JExpr::Name("__parentField".into()),
                            JExpr::Name("__ptr".into()),
                        ],
                    }),
                ],
            })
        }
    }

    /// Generate Java code for a getter.
    pub fn java_getter(&self) -> JMember {
        let name = format!("jni_get_{}", &self.name);

        JMember::ExternMethod(JExternMethod {
            name,
            is_static: true,
            private: true,
            ret: if_else!(self.is_primitive(), self.ty.j_type(), JType::Long),
            args: vec![("ptr".into(), JType::Long)],
        })
    }

    /// Generate Java wrapper code for a getter.
    pub fn java_getter_wrapper(&self) -> JMember {
        let field = &self.name;
        let native = format!("jni_get_{}", &self.name);
        let name = format!("get_{}", &self.name).to_case(Case::Camel);
        let ty = self.ty.full_type_java();

        if self.is_primitive() {
            JMember::Getter(JGetterImpl {
                name,
                getter_name: self.name.clone(),
                ret: self.ty.j_type(),
                args: Vec::new(),
                private: false,
                is_static: false,
                is_override: false,
                generics: BTreeMap::new(),
                code: vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                    target: native,
                    args: vec![JExpr::Name("__ptr".into())],
                })))],
            })
        } else {
            JMember::Getter(JGetterImpl {
                name,
                getter_name: self.name.clone(),
                ret: self.ty.j_type(),
                args: Vec::new(),
                private: false,
                is_static: false,
                is_override: false,
                generics: BTreeMap::new(),
                code: vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                    target: format!("{}.from", ty),
                    args: vec![
                        JExpr::Call(JCall {
                            target: native,
                            args: vec![JExpr::Name("__ptr".into())],
                        }),
                        JExpr::Name("this".into()),
                        JExpr::Name(format!("\"{field}\"")),
                    ],
                })))],
            })
        }
    }

    /// Generate Rust code for a setter.
    pub fn rust_setter(&self, cx: &ClassCtx) -> String {
        let name = cx.method_name(format!("jni_set_{}", &self.name));
        let class = cx.name_generics();
        let field = &self.name;

        let generics = cx
            .generics
            .iter()
            .map(|v| v.code())
            .collect::<Vec<_>>()
            .join(", ");

        if self.ty.kind.is_number() {
            let val_ty = self.ty.kind.jni_name();

            format!(
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
        let class = cx.name_generics();
        let field = &self.name;
        let ret = self.ty.kind.jni_name();

        let generics = cx
            .generics
            .iter()
            .map(|v| v.code())
            .collect::<Vec<_>>()
            .join(", ");

        if self.ty.kind.is_number() {
            format!(
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
                "{RUST_BRIDGE_HEAD}
pub unsafe extern \"system\" fn Java_{name}<'local, {generics}>(
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
