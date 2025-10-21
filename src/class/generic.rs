//! Type generics

use super::ty::Type;

/// A type generic
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeGeneric {
    /// The generic name.
    pub name: String,

    /// A list of bounds
    pub bounds: Vec<Type>,

    /// If the bounds should be rust-only.
    pub rust_only: bool,

    /// If there should be no bounds on the java side.
    pub free: bool,
}

impl TypeGeneric {
    /// Convert this into Rust code.
    pub fn code(&self) -> String {
        format!(
            "{}: {}",
            self.name,
            self.bounds
                .iter()
                .map(|v| v.full_type())
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

impl From<Type> for TypeGeneric {
    fn from(value: Type) -> Self {
        Self {
            name: value.kind.rust_name(),
            bounds: Vec::new(),
            rust_only: false,
            free: false,
        }
    }
}
