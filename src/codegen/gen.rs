//! The generation context module.

/// A context for codegen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Generator {
    /// The package to generate for.
    pub package: String,

    /// Whether to use JetBrains annotations.
    pub with_annotations: bool,

    /// The library file base name.
    pub library: String,
}

impl Generator {
    /// Convert the [`Self::package`] into the JNI function name equivalent
    pub fn jni_pkg(&self) -> String {
        self.package.replace('.', "_")
    }

    /// Convert the [`Self::package`] into the folder name equivalent
    pub fn dir_pkg(&self) -> String {
        self.package.replace('.', "/")
    }
}
