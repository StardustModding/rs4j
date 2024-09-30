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
    /// Create a new [`Type`].
    pub fn new(kind: TypeKind, generics: Option<Vec<Type>>) -> Self {
        Self { kind, generics }
    }

    /// Get the full Rust type.
    pub fn full_type(&self) -> String {
        let g = if_else!(
            self.generics.is_some(),
            format!("<{}>", self.get_generics()),
            "".into()
        );

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
        let g = if_else!(
            self.generics.is_some(),
            format!("<{}>", self.get_generics_java()),
            "".into()
        );

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

    /// Get the conversion function for this type
    pub fn convert_func(&self) -> String {
        self.kind.convert_func()
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
}

impl TypeKind {
    /// Is this type primitive?
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Other(_) => false,
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
        }
    }

    /// Can we cast from a JNI type to the Rust type?
    pub fn can_cast(&self) -> bool {
        match self {
            Self::String | Self::Other(_) => false,
            _ => true,
        }
    }

    /// Get the type's name in Java.
    pub fn java_name(&self) -> String {
        match self {
            Self::Void => "void".into(),
            Self::String => "String".into(),
            Self::I8 | Self::U8 => "byte".into(),
            Self::I16 | Self::U16 => "short".into(),
            Self::I32 | Self::U32 => "int".into(),
            Self::I64 | Self::U64 => "long".into(),
            Self::F32 => "float".into(),
            Self::F64 => "double".into(),
            Self::Bool => "boolean".into(),
            Self::Char => "char".into(),
            Self::Other(o) => o.to_owned(),
        }
    }

    /// Get the type's name in Java.
    pub fn native_name(&self) -> String {
        match self {
            Self::Other(_) => "long".into(),
            _ => self.java_name(),
        }
    }

    /// Is this type a number?
    pub fn is_number(&self) -> bool {
        match self {
            Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::F32
            | Self::F64
            | Self::Char => true,
            _ => false,
        }
    }

    /// Get the type for JNI.
    pub fn jni_name(&self) -> String {
        match self {
            Self::Void => "()".into(),
            Self::String => "jstring".into(),
            Self::I8 | Self::U8 => "jbyte".into(),
            Self::I16 | Self::U16 => "jshort".into(),
            Self::I32 | Self::U32 => "jint".into(),
            Self::I64 | Self::U64 => "jlong".into(),
            Self::F32 => "jfloat".into(),
            Self::F64 => "jdouble".into(),
            Self::Bool => "jboolean".into(),
            Self::Char => "jchar".into(),
            Self::Other(_) => "jlong".into(),
        }
    }

    /// Get the type for JNI arguments. For some reason, the string one is different.
    pub fn jni_arg_name(&self) -> String {
        match self {
            Self::String => "JString<'local>".into(),
            _ => self.jni_name(),
        }
    }

    /// Get the conversion function for this type
    pub fn convert_func(&self) -> String {
        match self {
            Self::Void => "Blaze3D.youJustLostTheGame".into(),
            Self::String => "NativeTools.getString".into(),
            Self::I8 | Self::U8 => "NativeTools.getByte".into(),
            Self::I16 | Self::U16 => "NativeTools.getShort".into(),
            Self::I32 | Self::U32 => "NativeTools.getInt".into(),
            Self::I64 | Self::U64 => "NativeTools.getLong".into(),
            Self::F32 => "NativeTools.getFloat".into(),
            Self::F64 => "NativeTools.getDouble".into(),
            Self::Bool => "NativeTools.getBool".into(),
            Self::Char => "NativeTools.getChar".into(),
            Self::Other(it) => format!("{}.from", it),
        }
    }
}
