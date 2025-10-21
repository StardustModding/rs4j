//! The module for [`Class`]es.

use base::{free_method_java, free_method_java_wrapper, free_method_rust, of_func};
use convert_case::{Case, Casing};
use ctx::ClassCtx;
use field::Field;
use generic::TypeGeneric;
use method::Method;
use std::collections::BTreeMap;

use crate::{
    class::{base::RUST_BRIDGE_HEAD_MANGLE, ty::Type},
    codegen::{
        cx::Generator,
        java::{
            JCall, JClassDef, JCtor, JExpr, JField, JGetterImpl, JGetterSetterImpl, JIf, JMember, JMethodImpl, JNewCall, JSetField, JType
        },
    },
    if_else,
};

pub mod arg;
pub mod base;
pub mod conv;
pub mod ctx;
pub mod expr;
pub mod field;
pub mod generic;
pub mod method;
pub mod native;
pub mod ty;
pub mod wrapper;

/// A Java class.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class {
    /// The name of this class.
    pub name: String,

    /// The package
    pub package: String,

    /// A list of imports.
    pub imports: Vec<String>,

    /// A list of fields.
    pub fields: Vec<Field>,

    /// A list of methods.
    pub methods: Vec<Method>,

    /// A list of generics.
    pub generics: Vec<TypeGeneric>,

    /// Should it be a wrapper?
    pub wrapped: bool,

    pub real_name: Option<(String, Vec<Type>)>,
}

impl Class {
    /// Create a new [`Class`].
    pub fn new(name: impl AsRef<str>, package: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().into(),
            package: package.as_ref().into(),
            imports: Self::default_imports(),
            fields: Vec::new(),
            methods: Vec::new(),
            generics: Vec::new(),
            wrapped: false,
            real_name: None,
        }
    }

    /// Get default imports.
    pub fn default_imports() -> Vec<String> {
        vec![
            "java.util.*".into(),
            "org.stardustmodding.rs4j.util.NativeTools".into(),
            "org.stardustmodding.rs4j.util.ParentClass".into(),
            "org.stardustmodding.rs4j.util.NativeClass".into(),
        ]
    }

    /// Set the package name.
    pub fn set_package(mut self, pkg: impl AsRef<str>) -> Self {
        self.package = pkg.as_ref().to_string();
        self
    }

    /// Add an import.
    pub fn import(mut self, import: impl AsRef<str>) -> Self {
        self.imports.push(import.as_ref().to_string());
        self
    }

    /// Add a field.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add a native method.
    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method);
        self
    }

    /// Create the Java code.
    pub fn java_code(&self, gcx: &Generator) -> JClassDef {
        let pkg = &self.package;
        let class = &self.name;
        let cx = self.new_context();
        let mut natives = Vec::new();
        let mut wrappers = Vec::new();
        let mut fields = Vec::new();
        let mut update_fields = Vec::new();
        let class_g = cx.raw_name_generics_java(gcx.kotlin);
        let wheres = cx.kotlin_wheres();
        let class_ge = cx.raw_name_generics();

        let generics = self
            .generics
            .iter()
            .map(|it| {
                (
                    it.name.clone(),
                    if it.free {
                        vec![]
                    } else if it.rust_only {
                        vec!["ParentClass".into(), "NativeClass".into()]
                    } else {
                        it.bounds
                            .iter()
                            .map(|it| it.j_type().name(gcx))
                            .collect::<Vec<_>>()
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();

        for func in &self.methods {
            natives.push(func.native_java_code());
        }

        for func in &self.methods {
            wrappers.push(func.wrapper_java_code(&cx));
        }

        for field in &self.fields {
            if field.rust {
                continue;
            }

            let name = &field.name;

            natives.push(field.java_setter());
            natives.push(field.java_getter());

            fields.push(JMember::GetterSetter(JGetterSetterImpl {
                name: field.name.clone(),
                setter_name: format!("set_{}", &field.name).to_case(Case::Camel),
                getter_name: format!("get_{}", &field.name).to_case(Case::Camel),
                is_static: false,
                private: false,
                ty: field.ty.clone(),
            }));

            // fields.push(field.java_setter_wrapper());
            // fields.push(field.java_getter_wrapper());

            if !field.is_primitive() {
                update_fields.push(JExpr::If(JIf {
                    cond: Box::new(JExpr::Name(format!("field == \"{name}\""))),
                    body: vec![JExpr::SetField(JSetField {
                        target: "__ptr".into(),
                        value: Box::new(JExpr::Call(JCall {
                            target: format!("jni_set_{name}"),
                            args: vec![JExpr::Name("__ptr".into()), JExpr::Name("pointer".into())],
                        })),
                    })],
                }));
            }
        }

        natives.push(free_method_java());
        wrappers.push(free_method_java_wrapper());

        let vars = vec![
            JMember::Field(JField {
                name: "__ptr".into(),
                is_final: false,
                is_static: false,
                private: true,
                ty: JType::Long,
                value: Some("-1".into()),
            }),
            JMember::Field(JField {
                name: "__parent".into(),
                is_final: false,
                is_static: false,
                private: true,
                ty: JType::Nullable(Box::new(JType::Custom("ParentClass".into()))),
                value: Some("null".into()),
            }),
            JMember::Field(JField {
                name: "__parentField".into(),
                is_final: false,
                is_static: false,
                private: true,
                ty: JType::Nullable(Box::new(JType::String)),
                value: Some("null".into()),
            }),
        ];

        let inits = vec![
            JMember::Ctor(JCtor {
                name: class.clone(),
                args: vec![("ptr".into(), JType::Long)],
                private: true,
                code: vec![JExpr::SetField(JSetField {
                    target: "__ptr".into(),
                    value: Box::new(JExpr::Name("ptr".into())),
                })],
            }),
            JMember::Ctor(JCtor {
                name: class.clone(),
                args: vec![
                    ("ptr".into(), JType::Long),
                    ("parent".into(), JType::Custom("ParentClass".into())),
                    ("parentField".into(), JType::String),
                ],
                private: true,
                code: vec![
                    JExpr::SetField(JSetField {
                        target: "__ptr".into(),
                        value: Box::new(JExpr::Name("ptr".into())),
                    }),
                    JExpr::SetField(JSetField {
                        target: "__parent".into(),
                        value: Box::new(JExpr::Name("parent".into())),
                    }),
                    JExpr::SetField(JSetField {
                        target: "__parentField".into(),
                        value: Box::new(JExpr::Name("parentField".into())),
                    }),
                ],
            }),
        ];

        let froms = vec![
            JMember::MethodImpl(JMethodImpl {
                name: "from".into(),
                args: vec![("ptr".into(), JType::Long)],
                ret: JType::Custom(class_ge.clone()), // TODO: Use proper generic types here
                generics: generics.clone(),
                is_override: false,
                is_static: true,
                private: false,
                code: vec![JExpr::Return(Box::new(JExpr::New(JNewCall {
                    target: class_ge.clone(),
                    args: vec![JExpr::Name("ptr".into())],
                })))],
            }),
            JMember::MethodImpl(JMethodImpl {
                name: "from".into(),
                args: vec![
                    ("ptr".into(), JType::Long),
                    ("parent".into(), JType::Custom("ParentClass".into())),
                    ("parentField".into(), JType::String),
                ],
                ret: JType::Custom(class_ge.clone()), // TODO: Use proper generic types here
                generics: generics.clone(),
                is_override: false,
                is_static: true,
                private: false,
                code: vec![JExpr::Return(Box::new(JExpr::New(JNewCall {
                    target: class_ge.clone(),
                    args: vec![
                        JExpr::Name("ptr".into()),
                        JExpr::Name("parent".into()),
                        JExpr::Name("parentField".into()),
                    ],
                })))],
            }),
        ];

        let overrides = vec![
            JMember::Getter(JGetterImpl {
                name: "getPointer".into(),
                getter_name: "pointer".into(),
                is_override: true,
                is_static: false,
                private: false,
                ret: JType::Long,
                args: Vec::new(),
                generics: BTreeMap::new(),
                code: vec![JExpr::Return(Box::new(JExpr::Name("__ptr".into())))],
            }),
            JMember::MethodImpl(JMethodImpl {
                name: "updateField".into(),
                is_override: true,
                is_static: false,
                private: false,
                ret: JType::Void,
                args: vec![
                    ("field".into(), JType::Nullable(Box::new(JType::String))),
                    ("pointer".into(), JType::Long),
                ],
                generics: BTreeMap::new(),
                code: update_fields,
            }),
        ];

        let mut members = Vec::new();

        members.extend(natives);
        members.extend(vars);
        members.extend(wrappers);
        members.extend(fields);
        members.extend(inits);
        members.extend(froms);
        members.extend(overrides);

        JClassDef {
            pkg: pkg.into(),
            name: class_g,
            extends: vec!["ParentClass".into(), "NativeClass".into()],
            members,
            imports: self.imports.clone(),
            wheres,
        }
    }

    /// Generate rust bindgen code
    pub fn rust_code(&self) -> String {
        let cx = self.new_context();
        let mut code = Vec::new();

        for f in &self.fields {
            if f.rust {
                continue;
            }

            code.push(f.rust_setter(&cx));
            code.push(f.rust_getter(&cx));
        }

        for m in &self.methods {
            code.push(m.native_rust_code(&cx, &self.fields, &self.generics));
        }

        code.push(free_method_rust(&cx, &self.fields));

        format!("{}\n{}", self.create_wrapper(), code.join("\n\n"))
    }

    /// Create the Rust code for a wrapper struct.
    pub fn create_wrapper(&self) -> String {
        let cx = self.new_context();
        let mut fields = Vec::new();

        let generics = self
            .generics
            .iter()
            .map(|v| v.code())
            .collect::<Vec<_>>()
            .join(", ");

        let generics = if_else!(generics != "", format!("<{}>", generics), "".into());

        let generics_nb = self
            .generics
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let generics_nb = if_else!(generics_nb != "", format!("<{}>", generics_nb), "".into());

        if self.wrapped {
            fields.push(format!("    pub __inner: {},", &cx.raw_name_generics()));
        }

        for field in &self.fields {
            if field.is_primitive() {
                fields.push(format!("    pub {}: {},", field.name, field.ty.full_type()));
            } else {
                fields.push(format!(
                    "    pub {}: *mut {},",
                    field.name,
                    field.ty.full_type()
                ));
            }
        }

        let struct_ = format!(
            "#[allow(non_camel_case_types)]\npub struct __JNI_{}{} {{\n{}\n}}",
            self.name,
            generics,
            fields.join("\n")
        );

        let mut impls = Vec::new();
        let mut convert = Vec::new();

        for field in &self.fields {
            if field.is_primitive() {
                convert.push(format!(
                    "            {}: self.{}.clone(),",
                    field.name, field.name
                ));
            } else {
                convert.push(format!(
                    "            {}: (&mut *self.{}).clone(),",
                    field.name, field.name
                ));
            }
        }

        impls.push(of_func(&cx, &self.fields));

        if self.wrapped {
            impls.push(format!(
                "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn to_rust(&self) -> {}{generics_nb} {{\n        self.__inner.clone()\n    }}",
                self.name,
            ));
        } else {
            impls.push(format!(
                "    {RUST_BRIDGE_HEAD_MANGLE}\n    pub unsafe fn to_rust(&self) -> {}{generics_nb} {{\n        {} {{\n{}\n        }}\n    }}",
                self.name,
                self.name,
                convert.join("\n")
            ));
        }

        for native in &self.methods {
            impls.push(native.native_rust_wrapper_code(&cx));
        }

        let impl_ = format!(
            "impl{} __JNI_{}{} {{\n{}\n}}\n",
            generics,
            self.name,
            generics_nb,
            impls.join("\n\n")
        );

        format!("{}\n\n{}", struct_, impl_)
    }

    /// Create a new [`ClassCtx`]
    fn new_context(&self) -> ClassCtx {
        ClassCtx::new(self)
    }
}
