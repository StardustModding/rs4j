//! The module for [`BoundExpr`]s.

use anyhow::Result;

use super::expr::Expr;

/// A type bound.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BoundExpr {
    /// The name of the type parameter.
    pub name: Box<Expr>,

    /// The traits for this type.
    pub traits: String,
}

impl BoundExpr {
    /// Convert this into Rust code.
    pub fn bounds(&self) -> Result<String> {
        Ok(format!("{}: {}", self.name.ident_strict()?, self.traits,))
    }
}
