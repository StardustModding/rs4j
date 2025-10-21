//! The generation context module.

use anyhow::Result;
use std::{fs, path::PathBuf};

/// A context for codegen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Generator {
    /// The package to generate for.
    pub package: String,

    /// Whether to use JetBrains annotations.
    pub with_annotations: bool,

    /// The library file base name.
    pub library: String,

    /// Whether we are generating Kotlin code or not.
    pub kotlin: bool,

    /// The output bindings (Java) directory.
    pub out_dir: PathBuf,
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

    /// Emit a class (name in the format of `com.example.Class`) to the output directory.
    pub fn emit(&self, name: impl AsRef<str>, code: impl AsRef<str>) -> Result<()> {
        let ext = if self.kotlin { "kt" } else { "java" };

        let pat = self
            .out_dir
            .join(format!("{}.{}", name.as_ref().replace('.', "/"), ext));

        let parent = pat.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        fs::write(pat, code.as_ref())?;

        Ok(())
    }
}
