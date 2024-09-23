//! The module for [`Expr`]s.

use anyhow::Result;

use crate::prelude::{IntoJavaType, RustTypes};

use super::{bound::BoundExpr, class::ClassExpr, field::FieldExpr, func::FunctionExpr, ty::TypeExpr};

/// An expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Expr {
    /// An identifier.
    Identifier(String),

    /// A [`FunctionExpr`].
    Function(FunctionExpr),

    /// A [`ClassExpr`].
    Class(ClassExpr),

    /// A [`TypeExpr`].
    Type(TypeExpr),

    /// A [`BoundExpr`].
    Bound(BoundExpr),

    /// A [`FieldExpr`].
    Field(FieldExpr),

    /// A catch-all.
    #[default]
    None,
}

impl Expr {
    /// get this as an identifier ([`String`]).
    pub fn ident(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else if let Self::Type(val) = self {
            if let Some(generics) = *val.generics.clone() {
                let generics = generics
                    .iter()
                    .map(|v| v.ident().unwrap())
                    .collect::<Vec<String>>()
                    .join(", ");

                Ok(format!("{}<{}>", val.id.ident()?, generics))
            } else {
                Ok(val.id.ident()?)
            }
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    /// Get this as an identifier for Java.
    pub fn ident_java(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(RustTypes::from(val.clone().as_str()).into_java_type())
        } else if let Self::Type(val) = self {
            if let Some(generics) = *val.generics.clone() {
                let generics = generics
                    .iter()
                    .map(|v| v.ident_java().unwrap())
                    .collect::<Vec<String>>()
                    .join(", ");

                Ok(format!("{}<{}>", val.id.ident_java()?, generics))
            } else {
                Ok(val.id.ident_java()?)
            }
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    /// Get this as an identifier, but without resolving it if this is a [`TypeExpr`].
    pub fn ident_strict(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    /// Get only the identifier without generics.
    pub fn ident_only(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else if let Self::Type(val) = self {
            Ok(val.id.ident_only()?)
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    /// Get this as an identifier, but without resolving it if this is a [`TypeExpr`], for Java.
    pub fn ident_strict_java(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(RustTypes::from(val.clone().as_str()).into_java_type())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    /// Get the [`TypeExpr`] out of this if it is one.
    pub fn get_type(&self) -> Result<TypeExpr> {
        if let Self::Type(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Type(_), got {:?}", self))
        }
    }
}
