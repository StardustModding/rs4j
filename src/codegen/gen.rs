use anyhow::Result;

use crate::parser::{class::ClassExpr, expr::Expr};

use super::rust::gen_function;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Generator {
    pub package: String,
    pub with_annotations: bool,
}

impl Generator {
    pub fn jni_pkg(&self) -> String {
        self.package.replace('.', "_")
    }

    pub fn dir_pkg(&self) -> String {
        self.package.replace('.', "/")
    }
}

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
