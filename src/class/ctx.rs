//! Class codegen context

use crate::if_else;

use super::{generic::TypeGeneric, JavaClassBuilder};

/// A codegen context for classes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassCtx {
    /// The class name
    pub name: String,

    /// The package name
    pub package: String,

    /// Generics
    pub generics: Vec<TypeGeneric>,
}

impl ClassCtx {
    /// Create a new [`ClassCtx`].
    pub fn new(class: &JavaClassBuilder) -> Self {
        Self {
            name: class.name.clone(),
            package: class.package.clone(),
            generics: class.generics.clone(),
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
}
