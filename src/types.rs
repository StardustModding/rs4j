pub trait IntoJavaType {
    fn into_java_type(&self) -> String;
}

macro_rules! from_type {
    ($ty: ident, $name: ident) => {
        impl From<$ty> for RustTypes {
            fn from(val: $ty) -> RustTypes {
                RustTypes::$name(val)
            }
        }

        impl Into<$ty> for RustTypes {
            fn into(self) -> $ty {
                if let Self::$name(val) = self {
                    val
                } else {
                    panic!("Expected RustTypes::{}, got {:?}", stringify!($name), self)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum RustTypes {
    String(String),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Other(String),

    #[default]
    Void,
}

from_type!(String, String);
from_type!(u8, Uint8);
from_type!(u16, Uint16);
from_type!(u32, Uint32);
from_type!(u64, Uint64);
from_type!(i8, Int8);
from_type!(i16, Int16);
from_type!(i32, Int32);
from_type!(i64, Int64);
from_type!(f32, Float32);
from_type!(f64, Float64);

impl From<&str> for RustTypes {
    fn from(val: &str) -> Self {
        match val {
            "String" | "str" => Self::String(String::new()),
            "i8" => Self::Int8(0),
            "i16" => Self::Int16(0),
            "i32" => Self::Int32(0),
            "i64" => Self::Int64(0),
            "u8" => Self::Uint8(0),
            "u16" => Self::Uint16(0),
            "u32" => Self::Uint32(0),
            "u64" => Self::Uint64(0),
            "f32" => Self::Float32(0.0),
            "f64" => Self::Float64(0.0),
            v => Self::Other(v.to_string()),
        }
    }
}

impl From<()> for RustTypes {
    fn from(_: ()) -> Self {
        Self::Void
    }
}

impl Into<()> for RustTypes {
    fn into(self) -> () {
        if let Self::Void = self {
            ()
        } else {
            panic!("Expected RustTypes::Void, got {:?}", self)
        }
    }
}

impl IntoJavaType for RustTypes {
    fn into_java_type(&self) -> String {
        match self.clone() {
            Self::String(_) => "String".to_string(),
            Self::Uint8(_) | Self::Int8(_) => "Byte".to_string(),
            Self::Uint16(_) | Self::Int16(_) => "Short".to_string(),
            Self::Uint32(_) | Self::Int32(_) => "Integer".to_string(),
            Self::Uint64(_) | Self::Int64(_) => "Long".to_string(),
            Self::Float32(_) => "Float".to_string(),
            Self::Float64(_) => "Double".to_string(),
            Self::Other(val) => val,
            Self::Void => "Void".to_string(),
        }
    }
}
