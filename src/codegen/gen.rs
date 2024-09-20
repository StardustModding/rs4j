//! The generation context module.

use anyhow::Result;

use crate::parser::{class::ClassExpr, expr::Expr};

use super::rust::gen_function;

/// A context for codegen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Generator {
    /// The package to generate for.
    pub package: String,

    /// Whether to use JetBrains annotations.
    pub with_annotations: bool,
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

/// Generate Rust functions for a class.
pub fn gen_class(gen: &Generator, class: ClassExpr) -> Result<String> {
    let mut data = String::new();
    let bounds = class.bounds()?;

    for item in *class.clone().stmts {
        if let Expr::Function(func) = item {
            data.push_str(&format!(
                "{}\n\n",
                gen_function(gen, &class, &bounds, func)?
            ));
        }
    }

    Ok(data)
}
