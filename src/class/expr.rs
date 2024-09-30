//! An [`Expr`].

use super::{field::Field, generic::TypeGeneric, method::Method};

/// An expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    /// A [`Method`].
    Method(Method),

    /// A [`TypeGeneric`].
    Generic(TypeGeneric),

    /// A [`Field`].
    Field(Field),

    /// Nothing.
    None,
}

impl Expr {
    /// Is this a method?
    pub fn is_method(&self) -> bool {
        if let Self::Method(_) = self {
            true
        } else {
            false
        }
    }

    /// Is this a generic?
    pub fn is_generic(&self) -> bool {
        if let Self::Generic(_) = self {
            true
        } else {
            false
        }
    }

    /// Is this a field?
    pub fn is_field(&self) -> bool {
        if let Self::Field(_) = self {
            true
        } else {
            false
        }
    }

    /// Is this none?
    pub fn is_none(&self) -> bool {
        self.clone() == Self::None
    }

    /// Get this as a method.
    pub fn get_method(&self) -> Option<Method> {
        if let Self::Method(m) = self {
            Some(m.clone())
        } else {
            None
        }
    }

    /// Get this as a generic.
    pub fn get_generic(&self) -> Option<TypeGeneric> {
        if let Self::Generic(g) = self {
            Some(g.clone())
        } else {
            None
        }
    }

    /// Get this as a field.
    pub fn get_field(&self) -> Option<Field> {
        if let Self::Field(f) = self {
            Some(f.clone())
        } else {
            None
        }
    }
}
