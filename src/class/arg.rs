//! Arguments

use crate::parser::{expr::Expr, ty::TypeExpr};

use super::ty::Type;

/// A function argument.
#[derive(Debug, Clone)]
pub struct FunctionArg {
    /// The argument name.
    pub name: String,

    /// The type of the argument.
    pub ty: Type,

    /// Should we borrow it?
    pub borrow: bool,

    /// Should it be mutable?
    pub mutable: bool,

    /// Does it need a `.into()`?
    pub into: bool,
}

impl FunctionArg {
    /// Get the reference for Java
    pub fn java_name(&self) -> String {
        if self.ty.kind.is_primitive() {
            self.name.clone()
        } else {
            format!("{}.getPointer()", self.name)
        }
    }
}

impl From<(Expr, TypeExpr, bool, bool, bool)> for FunctionArg {
    fn from((name, ty, borrow, mutable, into): (Expr, TypeExpr, bool, bool, bool)) -> Self {
        let name = name.ident().unwrap();

        Self {
            name,
            ty: ty.into(),
            borrow,
            mutable,
            into,
        }
    }
}
