//! Arguments

use crate::codegen::java::JExpr;

use super::ty::Type;

/// A function argument.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn java_name(&self) -> JExpr {
        if self.ty.kind.is_primitive() {
            JExpr::Name(self.name.clone())
        } else {
            JExpr::GetPointer(self.name.clone())
        }
    }
}
