use anyhow::Result;

use super::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BoundExpr {
    /// The name of the type parameter.
    pub name: Box<Expr>,

    /// The traits for this type.
    pub traits: Box<Vec<Expr>>,
}

impl BoundExpr {
    pub fn bounds(&self) -> Result<String> {
        Ok(format!(
            "{}: {}",
            self.name.ident_strict()?,
            self.traits
                .iter()
                .map(|v| v.ident().unwrap())
                .collect::<Vec<String>>()
                .join(" + ")
        ))
    }
}
