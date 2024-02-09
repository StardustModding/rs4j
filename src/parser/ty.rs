use anyhow::Result;

use crate::types::{IntoJavaType, RustTypes};

use super::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TypeExpr {
    /// The type's ID.
    pub id: Box<Expr>,

    /// The type's generics.
    pub generics: Box<Option<Vec<Expr>>>,
}

impl TypeExpr {
    pub fn as_java(&self) -> Result<String> {
        let ident = self.id.ident_strict()?;
        let java_type = RustTypes::from(ident.as_str()).into_java_type();

        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| {
                    v.ident_strict_java()
                        .unwrap_or(v.get_type().unwrap().as_java().unwrap())
                })
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", java_type, generics))
        } else {
            Ok(java_type)
        }
    }
}
