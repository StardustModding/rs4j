//! Types.

use crate::if_else;

/// A type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Type {
    /// The type kind.
    pub kind: TypeKind,

    /// The type generics.
    pub generics: Option<Vec<Type>>,
}

impl Type {
    /// Get the full Rust type.
    pub fn full_type(&self) -> String {
        let g = if_else!(self.generics.is_some(), format!("<{}>", self.get_generics()), "".into());

        format!("{}{}", self.kind.rust_name(), g)
    }

    /// Get the generics for this type.
    pub fn get_generics(&self) -> String {
        let mut generics = Vec::new();

        if let Some(items) = &self.generics {
            for g in items {
                generics.push(g.full_type());
            }
        }

        generics.join(", ")
    }

    /// Get the full Java type.
    pub fn full_type_java(&self) -> String {
        let g = if_else!(self.generics.is_some(), format!("<{}>", self.get_generics_java()), "".into());

        format!("{}{}", self.kind.java_name(), g)
    }

    /// Get the generics for this type for Java.
    pub fn get_generics_java(&self) -> String {
        let mut generics = Vec::new();

        if let Some(items) = &self.generics {
            for g in items {
                generics.push(g.full_type_java());
            }
        }

        generics.join(", ")
    }
}

/// A type kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TypeKind {
    /// A ([`unit`](https://doc.rust-lang.org/std/primitive.unit.html)) type (Java: void).
    #[default]
    Void,

    /// A [`String`] type (Java: `String`).
    String,
    
    /// An [`i8`] (Java: `byte`).
    I8,

    /// An [`i16`] (Java: `short`).
    I16,

    /// An [`i32`] (Java: `int`).
    I32,

    /// An [`i64`] (Java: `long`).
    I64,

    /// A [`u8`] (Java: `byte`, gets converted to [`i8`]).
    U8,

    /// A [`u16`] (Java: `short`, gets converted to [`i16`]).
    U16,

    /// A [`u32`] (Java: `int`, gets converted to [`i32`]).
    U32,

    /// A [`u64`] (Java: `long`, gets converted to [`i64`]).
    U64,

    /// A [`f32`] (Java: `float`).
    F32,

    /// A [`f64`] (Java: `double`).
    F64,

    /// A [`bool`] (Java: `boolean`).
    Bool,

    /// A [`char`] (Java: `char`).
    Char,

    /// A type that is non-primitive.
    Other(String),

    /// A type parameter.
    TypeParam(String),
}

impl TypeKind {
    /// Is this type primitive?
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Other(_) | Self::TypeParam(_) => false,
            _ => true,
        }
    }

    /// Get the type's name in Rust.
    pub fn rust_name(&self) -> String {
        match self {
            Self::Void => "()".into(),
            Self::String => "String".into(),
            Self::I8 => "i8".into(),
            Self::I16 => "i16".into(),
            Self::I32 => "i32".into(),
            Self::I64 => "i64".into(),
            Self::U8 => "u8".into(),
            Self::U16 => "u16".into(),
            Self::U32 => "u32".into(),
            Self::U64 => "u64".into(),
            Self::F32 => "f32".into(),
            Self::F64 => "f64".into(),
            Self::Bool => "bool".into(),
            Self::Char => "char".into(),
            Self::Other(o) => o.to_owned(),
            Self::TypeParam(t) => t.to_owned(),
        }
    }

    /// Get the type's name in Java.
    pub fn java_name(&self) -> String {
        match self {
            Self::Void => "Void".into(),
            Self::String => "String".into(),
            Self::I8 => "Byte".into(),
            Self::I16 => "Short".into(),
            Self::I32 => "Integer".into(),
            Self::I64 => "Long".into(),
            Self::U8 => "Byte".into(),
            Self::U16 => "Short".into(),
            Self::U32 => "Integer".into(),
            Self::U64 => "Long".into(),
            Self::F32 => "Float".into(),
            Self::F64 => "Double".into(),
            Self::Bool => "Boolean".into(),
            Self::Char => "Char".into(),
            Self::Other(o) => o.to_owned(),
            Self::TypeParam(t) => t.to_owned(),
        }
    }
}
