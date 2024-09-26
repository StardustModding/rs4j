//! The module for [`ClassExpr`]s.

use anyhow::Result;

use super::{bound::BoundExpr, expr::Expr, ty::TypeExpr};

/// A class.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ClassExpr {
    /// The name of the class.
    pub name: Box<Expr>,

    /// Should it be a wrapper?
    pub wrapped: bool,

    /// The real rust of the class.
    pub real_name: Box<(Expr, Option<Vec<TypeExpr>>)>,

    /// The statements in the class.
    pub stmts: Box<Vec<Expr>>,

    /// The class's generics.
    pub generics: Option<Vec<TypeExpr>>,
}

impl ClassExpr {
    /// Get the name/ident of this class as a [`String`].
    pub fn ident(&self) -> Result<String> {
        let ident = self.name.ident()?;

        if let Some(generics) = self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.as_rust().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", ident, generics))
        } else {
            Ok(ident)
        }
    }

    /// Get the name/ident of this class for Rust as a [`String`].
    pub fn ident_rust(&self) -> Result<String> {
        let mut id = self.real_name.0.ident()?;

        if let Some(generics) = self.real_name.1.clone() {
            let it = generics
                .iter()
                .map(|v| v.as_rust().unwrap())
                .collect::<Vec<_>>()
                .join(", ");

            id.push_str(&format!("<{}>", it));
        }

        Ok(id)
    }

    /// Get the generics for this class.
    pub fn generics(&self) -> String {
        if let Some(generics) = self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.id.ident_strict().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            generics
        } else {
            String::new()
        }
    }

    /// Get the type bounds for this class.
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
