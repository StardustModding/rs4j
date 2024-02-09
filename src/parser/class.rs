use anyhow::Result;

use super::{bound::BoundExpr, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ClassExpr {
    /// The name of the class.
    pub name: Box<Expr>,

    /// The statements in the class.
    pub stmts: Box<Vec<Expr>>,

    /// The class's generics.
    pub generics: Box<Option<Vec<Expr>>>,
}

impl ClassExpr {
    pub fn ident(&self) -> Result<String> {
        let ident = self.name.ident()?;

        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.ident().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", ident, generics))
        } else {
            Ok(ident)
        }
    }

    pub fn generics(&self) -> String {
        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.ident_only().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            generics
        } else {
            String::new()
        }
    }

    pub fn bounds(&self) -> Result<String> {
        let mut bounds = String::new();

        let list = self
            .stmts
            .iter()
            .filter(|v| if let Expr::Bound(_) = v { true } else { false })
            .map(|v| {
                if let Expr::Bound(bound) = v {
                    bound
                } else {
                    unreachable!()
                }
            })
            .cloned()
            .collect::<Vec<BoundExpr>>();

        if !list.is_empty() {
            bounds.push_str(" where\n");

            for item in list {
                bounds.push_str(&format!("    {},\n", item.bounds()?));
            }
        }

        Ok(bounds)
    }
}
