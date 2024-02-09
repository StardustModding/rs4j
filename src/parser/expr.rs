use anyhow::Result;

use crate::types::{IntoJavaType, RustTypes};

use super::{bound::BoundExpr, class::ClassExpr, func::FunctionExpr, ty::TypeExpr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Expr {
    Identifier(String),
    Function(FunctionExpr),
    Class(ClassExpr),
    Type(TypeExpr),
    Bound(BoundExpr),

    #[default]
    None,
}

impl Expr {
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

    pub fn ident_strict(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn ident_only(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(val.clone())
        } else if let Self::Type(val) = self {
            Ok(val.id.ident_only()?)
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn ident_strict_java(&self) -> Result<String> {
        if let Self::Identifier(val) = self {
            Ok(RustTypes::from(val.clone().as_str()).into_java_type())
        } else {
            Err(anyhow!("Expected Self::Identifier(_), got {:?}", self))
        }
    }

    pub fn get_type(&self) -> Result<TypeExpr> {
        if let Self::Type(val) = self {
            Ok(val.clone())
        } else {
            Err(anyhow!("Expected Self::Type(_), got {:?}", self))
        }
    }
}
