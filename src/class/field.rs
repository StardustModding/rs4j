//! Fields.

use super::ty::Type;

/// A field in a class.
pub struct Field {
    /// The name of this field.
    pub name: String,

    /// This field's [`Type`].
    pub ty: Type,
}

impl Field {
    /// Create a new [`Field`].
    pub fn new(name: impl AsRef<str>, ty: Type) -> Self {
        Self {
            name: name.as_ref().into(),
            ty,
        }
    }
}
