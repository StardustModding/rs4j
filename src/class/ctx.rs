//! Class codegen context

use super::JavaClassBuilder;

/// A codegen context for classes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassCtx {
    /// The class name
    pub name: String,

    /// The package name
    pub package: String,
}

impl ClassCtx {
    /// Create a new [`ClassCtx`].
    pub fn new(class: &JavaClassBuilder) -> Self {
        Self {
            name: class.name.clone(),
            package: class.package.clone(),
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
}
