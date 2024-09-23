//! The module for [`FieldExpr`]s.

use super::expr::Expr;

/// A field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FieldExpr {
    /// The name of the field.
    pub name: Box<Expr>,

    /// The type of this field.
    pub ty: Box<Expr>,
}
