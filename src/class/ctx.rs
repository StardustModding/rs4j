//! Class codegen context

use crate::if_else;

use super::{Class, generic::TypeGeneric};

/// A codegen context for classes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassCtx {
    /// The class name
    pub name: String,

    /// The package name
    pub package: String,

    /// Generics
    pub generics: Vec<TypeGeneric>,

    /// Is the class a wrapper?
    pub wrapped: bool,
}

impl ClassCtx {
    /// Create a new [`ClassCtx`].
    pub fn new(class: &Class) -> Self {
        Self {
            name: class.name.clone(),
            package: class.package.clone(),
            generics: class.generics.clone(),
            wrapped: class.wrapped,
        }
    }

    /// The base name for Rust methods
    pub fn base_name(&self) -> String {
        format!("{}_{}", self.package.replace(".", "_"), &self.name)
    }

    /// Make a method name
    pub fn method_name(&self, method: impl AsRef<str>) -> String {
        format!(
            "{}_{}",
            self.base_name(),
            method.as_ref().replace("_", "_1")
        )
    }

    /// Get the name of the wrapper struct
    pub fn name(&self) -> String {
        format!("__JNI_{}", &self.name)
    }

    /// Get the name of the wrapper struct with generics
    pub fn name_generics(&self) -> String {
        let generics = self
            .generics
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let generics = if_else!(generics != "", format!("<{}>", generics), "".into());

        format!("__JNI_{}{}", &self.name, generics)
    }

    /// Get the name of the struct with generics
    pub fn raw_name_generics(&self) -> String {
        let generics = self
            .generics
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let generics = if_else!(generics != "", format!("<{}>", generics), "".into());

        format!("{}{}", &self.name, generics)
    }

    /// Get the name of the class with generics for Java
    pub fn raw_name_generics_java(&self, kotlin: bool) -> String {
        let generics = self
            .generics
            .iter()
            .map(|v| {
                format!(
                    "{} {}",
                    v.name,
                    if_else!(kotlin, "", "extends ParentClass & NativeClass")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let generics = if_else!(generics != "", format!("<{}>", generics), "".into());

        format!("{}{}", &self.name, generics)
    }

    pub fn kotlin_wheres(&self) -> String {
        let generics = self
            .generics
            .iter()
            .flat_map(|v| {
                if v.free {
                    vec![]
                } else {
                    vec![
                        format!("{}: ParentClass", v.name),
                        format!("{}: NativeClass", v.name),
                    ]
                }
            })
            .collect::<Vec<_>>();

        if generics.is_empty() {
            "".into()
        } else {
            format!(" where {}", generics.join(", "))
        }
    }

    /// Get generics for Java
    pub fn generics_java(&self) -> String {
        let generics = self
            .generics
            .iter()
            .map(|v| format!("{} extends ParentClass & NativeClass", v.name))
            .collect::<Vec<_>>()
            .join(", ");

        if_else!(generics != "", format!("<{}>", generics), "".into())
    }

    /// Get generics
    pub fn generics(&self) -> String {
        let generics = self
            .generics
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        if_else!(generics != "", format!("<{}>", generics), "".into())
    }
}
