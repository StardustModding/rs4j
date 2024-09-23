//! Arguments

use super::ty::Type;

/// A function argument.
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
