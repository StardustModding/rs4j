//! Type generics

use crate::parser::{bound::BoundExpr, ty::TypeExpr};

/// A type generic
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeGeneric {
    /// The generic name.
    pub name: String,

    /// A list of bounds
    pub bounds: Vec<String>,
}

impl From<BoundExpr> for TypeGeneric {
    fn from(value: BoundExpr) -> Self {
        Self {
            name: value.name.ident_strict().unwrap(),
            bounds: value
                .traits
                .split("+")
                .map(|v| v.trim().to_string())
                .collect(),
        }
    }
}

impl From<TypeExpr> for TypeGeneric {
    fn from(value: TypeExpr) -> Self {
        Self {
            name: value.id.ident_strict().unwrap(),
            bounds: Vec::new(),
        }
    }
}
