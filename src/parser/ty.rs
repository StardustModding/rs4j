//! The module for [`TypeExpr`]s.

use anyhow::Result;

use crate::prelude::{IntoJavaType, RustTypes};

use super::expr::Expr;

/// A type (with optional generics).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeExpr {
    /// The type's ID.
    pub id: Box<Expr>,

    /// The type's generics.
    pub generics: Box<Option<Vec<TypeExpr>>>,
}

impl Default for TypeExpr {
    fn default() -> Self {
        Self {
            id: Box::new(Expr::Identifier("void".into())),
            generics: Box::new(None),
        }
    }
}

impl TypeExpr {
    /// Get this as a Java type.
    pub fn as_java(&self) -> Result<String> {
        let ident = self.id.ident_strict()?;
        let java_type = RustTypes::from(ident.as_str()).into_java_type();

        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.as_java().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", java_type, generics))
        } else {
            Ok(java_type)
        }
    }

    /// Get this as a Rust type.
    pub fn as_rust(&self) -> Result<String> {
        let ident = self.id.ident_strict()?;

        if let Some(generics) = *self.generics.clone() {
            let generics = generics
                .iter()
                .map(|v| v.as_rust().unwrap())
                .collect::<Vec<String>>()
                .join(", ");

            Ok(format!("{}<{}>", ident, generics))
        } else {
            Ok(ident)
        }
    }

    /// Get the conversion method
    pub fn conv_method(&self) -> Result<&'static str> {
        Ok(match self.id.ident()?.as_str() {
            "String" => "NativeTools.getString",
            "i8" | "u8" => "NativeTools.getByte",
            "i16" | "u16" => "NativeTools.getShort",
            "i32" | "u32" => "NativeTools.getInt",
            "i64" | "u64" => "NativeTools.getLong",
            "f32" => "NativeTools.getFloat",
            "f64" => "NativeTools.getDouble",
            "bool" => "NativeTools.getBool",
            "char" => "NativeTools.getChar",

            _ => "NativeTools.getObject",
        })
    }
}
