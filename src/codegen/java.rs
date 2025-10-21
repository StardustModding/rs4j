//! Java codegen.

use std::collections::BTreeMap;

use anyhow::Result;

use crate::{
    class::{Class, ty::Type},
    if_else,
    java::{java, kotlin},
    loader::generate_loader,
};

use super::cx::Generator;

/// Generate the Java code for an entire `.rs4j` file.
pub fn gen_java_code(cx: &Generator, classes: &Vec<Class>) -> Result<()> {
    for class in classes {
        cx.emit(
            format!("{}.{}", cx.package, class.name),
            class.java_code(cx).code(cx),
        )?;
    }

    cx.emit(format!("{}.NativeLoader", cx.package), generate_loader(&cx))?;

    cx.emit(
        "cz.adamh.utils.NativeUtils",
        if cx.kotlin {
            kotlin::NATIVE_UTILS
        } else {
            java::NATIVE_UTILS
        },
    )?;

    cx.emit(
        "org.stardustmodding.rs4j.util.NativeTools",
        if cx.kotlin {
            kotlin::NATIVE_TOOLS
        } else {
            java::NATIVE_TOOLS
        },
    )?;

    cx.emit(
        "org.stardustmodding.rs4j.util.ParentClass",
        if cx.kotlin {
            kotlin::PARENT_CLASS
        } else {
            java::PARENT_CLASS
        },
    )?;

    cx.emit(
        "org.stardustmodding.rs4j.util.NativeClass",
        if cx.kotlin {
            kotlin::NATIVE_CLASS
        } else {
            java::NATIVE_CLASS
        },
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
pub enum JType {
    Int,
    Float,
    Long,
    Double,
    Bool,
    Void,
    Byte,
    Short,
    Char,
    String,
    Custom(String),
    Nullable(Box<JType>),

    Generic {
        base: Box<JType>,
        params: Vec<JType>,
    },
}

impl JType {
    pub fn name(&self, cx: &Generator) -> String {
        match self {
            JType::Int => if cx.kotlin { "Int" } else { "int" }.into(),
            JType::Float => if cx.kotlin { "Float" } else { "float" }.into(),
            JType::Long => if cx.kotlin { "Long" } else { "long" }.into(),
            JType::Double => if cx.kotlin { "Double" } else { "double" }.into(),
            JType::Bool => if cx.kotlin { "Boolean" } else { "boolean" }.into(),
            JType::Void => if cx.kotlin { "Unit" } else { "void" }.into(),
            JType::Byte => if cx.kotlin { "Byte" } else { "byte" }.into(),
            JType::Short => if cx.kotlin { "Short" } else { "short" }.into(),
            JType::Char => if cx.kotlin { "Char" } else { "char" }.into(),
            JType::String => "String".into(),
            JType::Custom(it) => it.clone(),

            JType::Generic { base, params } => format!(
                "{}<{}>",
                base.name(cx),
                params
                    .iter()
                    .map(|it| it.name(cx))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            JType::Nullable(it) => {
                if cx.kotlin {
                    format!("{}?", it.name(cx))
                } else if cx.with_annotations {
                    format!("@org.jetbrains.annotations.Nullable {}", it.name(cx))
                } else {
                    it.name(cx)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct JExternMethod {
    pub name: String,
    pub ret: JType,
    pub args: Vec<(String, JType)>,
    pub private: bool,
    pub is_static: bool,
}

impl JExternMethod {
    pub fn code(&self, cx: &Generator) -> String {
        let JExternMethod {
            name,
            ret,
            args,
            private,
            is_static,
        } = self;

        let private_j = if_else!(*private, "private ", "public ");
        let private = if_else!(*private, "private ", "");
        let static_ = if_else!(*is_static, "static ", "");

        if cx.kotlin {
            let pfx = if_else!(*is_static, "@JvmStatic ", "");

            format!(
                "{pfx}{private}external fun {name}({}): {}",
                args.iter()
                    .map(|(name, ty)| format!("{name}: {}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
                ret.name(cx),
            )
        } else {
            format!(
                "{private_j}{static_}native {} {name}({});",
                ret.name(cx),
                args.iter()
                    .map(|(name, ty)| format!("{} {name}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JMethodImpl {
    pub name: String,
    pub ret: JType,
    pub args: Vec<(String, JType)>,
    pub private: bool,
    pub is_static: bool,
    pub code: Vec<JExpr>,
    pub is_override: bool,

    /// A map of generic names to bounds.
    pub generics: BTreeMap<String, Vec<String>>,
}

impl JMethodImpl {
    pub fn code(&self, cx: &Generator) -> String {
        let JMethodImpl {
            name,
            ret,
            args,
            private,
            is_static,
            code,
            is_override,
            generics,
        } = self;

        let ov_j = if_else!(*is_override, "@Override ", "");
        let ov = if_else!(*is_override, "override ", "");
        let private_j = if_else!(*private, "private ", "public ");
        let private = if_else!(*private, "private ", "");
        let static_ = if_else!(*is_static, "static ", "");

        let body = code
            .iter()
            .map(|it| it.code(cx).ensure_semi(!cx.kotlin))
            .collect::<Vec<_>>()
            .join("\n")
            .indent(4);

        if cx.kotlin {
            let wheres = generics
                .iter()
                .flat_map(|(name, bounds)| bounds.iter().map(move |it| format!("{name}: {it}")))
                .collect::<Vec<_>>();

            let generics = generics
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();

            let wheres = if wheres.is_empty() {
                "".into()
            } else {
                format!(" where {}", wheres.join(", "))
            };

            let generics = if generics.is_empty() {
                "".into()
            } else {
                format!(" <{}>", generics.join(", "))
            };

            format!(
                "{private}{ov}fun{generics} {name}({}): {}{wheres} {{\n{body}\n}}",
                args.iter()
                    .map(|(name, ty)| format!("{name}: {}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
                ret.name(cx),
            )
        } else {
            let generics = generics
                .iter()
                .map(|(name, bounds)| {
                    if bounds.is_empty() {
                        name.clone()
                    } else {
                        format!("{} extends {}", name, bounds.join(" + "))
                    }
                })
                .collect::<Vec<_>>();

            let generics = if generics.is_empty() {
                "".into()
            } else {
                format!("<{}> ", generics.join(", "))
            };

            format!(
                "{ov_j}{private_j}{static_}{generics}{} {name}({}) {{\n{body}\n}}",
                ret.name(cx),
                args.iter()
                    .map(|(name, ty)| format!("{} {name}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JGetterImpl {
    pub name: String,
    pub getter_name: String,
    pub ret: JType,
    pub args: Vec<(String, JType)>,
    pub private: bool,
    pub is_static: bool,
    pub code: Vec<JExpr>,
    pub is_override: bool,

    /// A map of generic names to bounds.
    pub generics: BTreeMap<String, Vec<String>>,
}

impl JGetterImpl {
    pub fn code(&self, cx: &Generator) -> String {
        let JGetterImpl {
            name,
            getter_name,
            ret,
            args,
            private,
            is_static,
            code,
            is_override,
            generics,
        } = self;

        let ov_j = if_else!(*is_override, "@Override ", "");
        let ov = if_else!(*is_override, "override ", "");
        let private_j = if_else!(*private, "private ", "public ");
        let private = if_else!(*private, "private ", "");
        let static_ = if_else!(*is_static, "static ", "");

        let body = code
            .iter()
            .map(|it| it.code(cx).ensure_semi(!cx.kotlin))
            .collect::<Vec<_>>()
            .join("\n")
            .indent(4);

        if cx.kotlin {
            let generics = generics
                .iter()
                .map(|(name, bounds)| {
                    if bounds.is_empty() {
                        name.clone()
                    } else {
                        format!("{} : {}", name, bounds.join(" + "))
                    }
                })
                .collect::<Vec<_>>();

            if !generics.is_empty() || !args.is_empty() {
                let generics = if generics.is_empty() {
                    "".into()
                } else {
                    format!(" <{}>", generics.join(", "))
                };

                format!(
                    "{private}{ov}fun{generics} {name}({}): {} {{\n{body}\n}}",
                    args.iter()
                        .map(|(name, ty)| format!("{name}: {}", ty.name(cx)))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret.name(cx),
                )
            } else {
                format!(
                    "{private}{ov}val {getter_name}: {} get() {{\n{body}\n}}",
                    ret.name(cx),
                )
            }
        } else {
            let generics = generics
                .iter()
                .map(|(name, bounds)| {
                    if bounds.is_empty() {
                        name.clone()
                    } else {
                        format!("{} extends {}", name, bounds.join(" + "))
                    }
                })
                .collect::<Vec<_>>();

            let generics = if generics.is_empty() {
                "".into()
            } else {
                format!(" <{}>", generics.join(", "))
            };

            format!(
                "{ov_j}{private_j}{static_}{generics}{} {name}({}) {{\n{body}\n}}",
                ret.name(cx),
                args.iter()
                    .map(|(name, ty)| format!("{} {name}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JGetterSetterImpl {
    /// The field name, used for Kotlin properties.
    pub name: String,

    /// The Java method name for the setter.
    pub setter_name: String,

    /// The Java method name for the getter.
    pub getter_name: String,
    pub ty: Type,
    pub private: bool,
    pub is_static: bool,
}

impl JGetterSetterImpl {
    pub fn code(&self, cx: &Generator) -> String {
        let JGetterSetterImpl {
            name,
            getter_name,
            setter_name,
            ty,
            private,
            is_static,
        } = self;

        let native_get = format!("jni_get_{}", &self.name);
        let native_set = format!("jni_set_{}", &self.name);

        let private = if_else!(*private, "private ", "");

        let set_code = if ty.kind.is_primitive() {
            vec![
                JExpr::SetField(JSetField {
                    target: "__ptr".into(),
                    value: Box::new(JExpr::Call(JCall {
                        target: native_set,
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
            ]
        } else {
            vec![
                JExpr::SetField(JSetField {
                    target: "__ptr".into(),
                    value: Box::new(JExpr::Call(JCall {
                        target: native_set,
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
            ]
        };

        let get_code = if ty.kind.is_primitive() {
            vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                target: native_get,
                args: vec![JExpr::Name("__ptr".into())],
            })))]
        } else {
            vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                target: format!("{}.from", ty.full_type_java()),
                args: vec![
                    JExpr::Call(JCall {
                        target: native_get,
                        args: vec![JExpr::Name("__ptr".into())],
                    }),
                    JExpr::Name("this".into()),
                    JExpr::Name(format!("\"{name}\"")),
                ],
            })))]
        };

        let set_body = set_code
            .iter()
            .map(|it| it.code(cx).ensure_semi(!cx.kotlin))
            .collect::<Vec<_>>()
            .join("\n")
            .indent(8);

        let get_body = get_code
            .iter()
            .map(|it| it.code(cx).ensure_semi(!cx.kotlin))
            .collect::<Vec<_>>()
            .join("\n")
            .indent(8);

        if cx.kotlin {
            format!(
                "@get:JvmName(\"{getter_name}\")\n@set:JvmName(\"{setter_name}\")\n{private}var {name}: {}\n    get() {{\n{get_body}\n    }}\n    set(value) {{\n{set_body}\n    }}",
                ty.j_type().name(cx),
            )
        } else {
            let set = JMethodImpl {
                args: vec![("value".into(), ty.j_type())],
                code: set_code,
                generics: BTreeMap::new(),
                is_override: false,
                is_static: *is_static,
                private: self.private,
                ret: ty.j_type(),
                name: self.setter_name.clone(),
            };

            let get = JMethodImpl {
                args: vec![],
                code: get_code,
                generics: BTreeMap::new(),
                is_override: false,
                is_static: *is_static,
                private: self.private,
                ret: ty.j_type(),
                name: self.getter_name.clone(),
            };

            format!("{}\n{}", set.code(cx), get.code(cx))
        }
    }
}

#[derive(Debug, Clone)]
pub struct JField {
    pub name: String,
    pub ty: JType,
    pub private: bool,
    pub is_static: bool,
    pub is_final: bool,
    pub value: Option<String>,
}

impl JField {
    pub fn code(&self, cx: &Generator) -> String {
        let JField {
            name,
            ty,
            private,
            is_static,
            is_final,
            value,
        } = self;

        let private_j = if_else!(*private, "private ", "public ");
        let final_j = if_else!(*is_final, "final ", "");
        let private = if_else!(*private, "private ", "");
        let final_ = if_else!(*is_final, "val ", "var ");
        let static_ = if_else!(*is_static, "static ", "");

        let value = if let Some(it) = value {
            format!(" = {it}")
        } else {
            "".into()
        };

        if cx.kotlin {
            format!("{private}{final_}{name}: {}{value}", ty.name(cx),)
        } else {
            format!(
                "{private_j}{static_}{final_j}{} {name}{value};",
                ty.name(cx),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JCtor {
    pub name: String,
    pub args: Vec<(String, JType)>,
    pub private: bool,
    pub code: Vec<JExpr>,
}

impl JCtor {
    pub fn code(&self, cx: &Generator) -> String {
        let JCtor {
            name,
            args,
            private,
            code,
        } = self;

        let private_j = if_else!(*private, "private ", "public ");
        let private = if_else!(*private, "private ", "");

        let body = code
            .iter()
            .map(|it| it.code(cx).ensure_semi(!cx.kotlin))
            .collect::<Vec<_>>()
            .join("\n")
            .indent(4);

        if cx.kotlin {
            format!(
                "{private}constructor({}) {{\n{body}\n}}",
                args.iter()
                    .map(|(name, ty)| format!("{name}: {}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        } else {
            format!(
                "{private_j}{name}({}) {{\n{body}\n}}",
                args.iter()
                    .map(|(name, ty)| format!("{} {name}", ty.name(cx)))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JCall {
    pub target: String,
    pub args: Vec<JExpr>,
}

impl JCall {
    pub fn code(&self, cx: &Generator) -> String {
        format!(
            "{}({})",
            self.target,
            self.args
                .iter()
                .map(|it| it.code(cx))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct JSafeFieldCall {
    pub field: String,
    pub target: String,
    pub args: Vec<JExpr>,
}

impl JSafeFieldCall {
    pub fn code(&self, cx: &Generator) -> String {
        if cx.kotlin {
            format!(
                "{}?.{}({})",
                self.field,
                self.target,
                self.args
                    .iter()
                    .map(|it| it.code(cx))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!(
                "if ({} != null) {}.{}({})",
                self.field,
                self.field,
                self.target,
                self.args
                    .iter()
                    .map(|it| it.code(cx))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct JNewCall {
    pub target: String,
    pub args: Vec<JExpr>,
}

impl JNewCall {
    pub fn code(&self, cx: &Generator) -> String {
        let n = if_else!(cx.kotlin, "", "new ");

        format!(
            "{n}{}({})",
            self.target,
            self.args
                .iter()
                .map(|it| it.code(cx))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct JSetField {
    pub target: String,
    pub value: Box<JExpr>,
}

impl JSetField {
    pub fn code(&self, cx: &Generator) -> String {
        let s = if_else!(cx.kotlin, "", ";");

        format!("{} = {}{s}", self.target, self.value.code(cx))
    }
}

#[derive(Debug, Clone)]
pub struct JIf {
    pub cond: Box<JExpr>,
    pub body: Vec<JExpr>,
}

impl JIf {
    pub fn code(&self, cx: &Generator) -> String {
        format!(
            "if ({}) {{\n{}\n}}",
            self.cond.code(cx),
            self.body
                .iter()
                .map(|it| it.code(cx))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(4)
        )
    }
}

#[derive(Debug, Clone)]
pub struct JVar {
    pub mutable: bool,
    pub name: String,
    pub ty: JType,
    pub value: Box<JExpr>,
}

impl JVar {
    pub fn code(&self, cx: &Generator) -> String {
        if cx.kotlin {
            let cmd = if_else!(self.mutable, "var ", "val ");

            format!(
                "{cmd}{}: {} = {}",
                self.name,
                self.ty.name(cx),
                self.value.code(cx)
            )
        } else {
            format!(
                "{} {} = {};",
                self.ty.name(cx),
                self.name,
                self.value.code(cx)
            )
        }
    }
}

#[derive(Debug, Clone)]
pub enum JExpr {
    Call(JCall),
    SetField(JSetField),
    Name(String),
    If(JIf),
    Var(JVar),
    Return(Box<JExpr>),
    New(JNewCall),
    SafeFieldCall(JSafeFieldCall),
    GetPointer(String),
}

impl JExpr {
    pub fn code(&self, cx: &Generator) -> String {
        match self {
            JExpr::Call(it) => it.code(cx),
            JExpr::New(it) => it.code(cx),
            JExpr::SetField(it) => it.code(cx),
            JExpr::If(it) => it.code(cx),
            JExpr::Name(it) => it.clone(),
            JExpr::Var(it) => it.code(cx),
            JExpr::Return(it) => format!("return {}{}", it.code(cx), if_else!(cx.kotlin, "", ";")),
            JExpr::SafeFieldCall(it) => it.code(cx),

            JExpr::GetPointer(it) => {
                if cx.kotlin {
                    format!("{it}.pointer")
                } else {
                    format!("{it}.getPointer()")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum JMember {
    MethodImpl(JMethodImpl),
    ExternMethod(JExternMethod),
    Field(JField),
    Ctor(JCtor),
    Getter(JGetterImpl),
    GetterSetter(JGetterSetterImpl),
}

impl JMember {
    pub fn code(&self, cx: &Generator) -> String {
        match self {
            JMember::MethodImpl(it) => it.code(cx),
            JMember::ExternMethod(it) => it.code(cx),
            JMember::Field(it) => it.code(cx),
            JMember::Ctor(it) => it.code(cx),
            JMember::Getter(it) => it.code(cx),
            JMember::GetterSetter(it) => it.code(cx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JClassDef {
    pub pkg: String,
    pub name: String,
    pub extends: Vec<String>,
    pub members: Vec<JMember>,
    pub imports: Vec<String>,
    pub wheres: String,
}

impl JClassDef {
    pub fn code(&self, cx: &Generator) -> String {
        if cx.kotlin {
            let mut data = Vec::new();

            data.push("@file:Suppress(\"FunctionName\", \"PrivatePropertyName\", \"unused\", \"RedundantUnitReturnType\", \"UnusedImport\")".into());
            data.push(format!("package {}", self.pkg));
            data.push("".into());

            if !self.imports.is_empty() {
                for item in &self.imports {
                    data.push(format!("import {item}"));
                }

                data.push("".into());
            }

            let ext = if self.extends.is_empty() {
                "".into()
            } else {
                format!(" : {}", self.extends.join(", "))
            };

            data.push(format!("class {}{ext}{} {{", self.name, self.wheres));

            let mut companion = Vec::new();

            for item in &self.members {
                match item {
                    JMember::ExternMethod(JExternMethod {
                        is_static: true, ..
                    })
                    | JMember::MethodImpl(JMethodImpl {
                        is_static: true, ..
                    })
                    | JMember::Field(JField {
                        is_static: true, ..
                    }) => companion.push(item.code(cx).indent(4)),

                    _ => data.push(item.code(cx).indent(4)),
                }
            }

            if !companion.is_empty() {
                data.push("    companion object {".into());
                data.push(companion.join("\n").indent(4));
                data.push("    }".into());
            }

            data.push("}".into());

            data.join("\n")
        } else {
            let mut data = Vec::new();

            data.push(format!("package {};", self.pkg));
            data.push("".into());

            if !self.imports.is_empty() {
                for item in &self.imports {
                    data.push(format!("import {item};"));
                }

                data.push("".into());
            }

            let ext = if self.extends.is_empty() {
                "".into()
            } else {
                format!(" implements {}", self.extends.join(", "))
            };

            data.push(format!("public class {}{ext} {{", self.name));

            for item in &self.members {
                data.push(item.code(cx).indent(4));
            }

            data.push("}".into());
            data.join("\n")
        }
    }
}

pub trait JUtil {
    fn indent(&self, amount: usize) -> String;
    fn ensure_semi(self, semi: bool) -> String;
}

impl JUtil for String {
    fn indent(&self, amount: usize) -> String {
        let indent = " ".repeat(amount);

        self.split("\n")
            .into_iter()
            .map(|it| format!("{}{}", indent, it))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ensure_semi(self, semi: bool) -> String {
        if semi && !self.ends_with(';') {
            format!("{self};")
        } else {
            self
        }
    }
}
