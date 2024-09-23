
#[allow(unused_imports)]
use jni::sys::{
    jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort, jstring,
    jvalue,
};

use jni::{
    objects::{JObject, JValueGen},
    JNIEnv,
};

/// Convert a [`jlong`] to a `*mut T`
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

/// Convert a [`jlong`] to a `*mut T`
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

/// Convert an object (`T`) to a [`jobject`]
#[allow(dead_code)]
pub fn object_to_jobject<T>(mut env: JNIEnv, obj: T, jcls: String) -> jobject {
    let jobj: JObject = env.alloc_object(jcls).unwrap();

    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");

    let ret: jlong = Box::into_raw(Box::new(obj)) as jlong;
    let res = env.set_field(&jobj, "__pointer", "Long", JValueGen::Long(ret));

    if env.exception_check().unwrap() || res.is_err() {
        panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
    }

    jobj.as_raw()
}



/// A trait for things that can be converted into Java types.
pub trait IntoJavaType {
    /// Convert this into a [`String`] (java type)
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
    };
}

/// An enum for Rust types.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum RustTypes {
    /// A [`String`].
    String(String),

    /// A [`bool`].
    Bool(bool),

    /// A [`u8`].
    Uint8(u8),

    /// A [`u16`].
    Uint16(u16),

    /// A [`u32`].
    Uint32(u32),

    /// A [`u64`].
    Uint64(u64),

    /// A [`u128`].
    Uint128(u128),

    /// An [`i8`].
    Int8(i8),

    /// An [`i16`].
    Int16(i16),

    /// An [`i32`].
    Int32(i32),

    /// An [`i64`].
    Int64(i64),

    /// An [`i128`].
    Int128(i128),

    /// A [`f32`].
    Float32(f32),

    /// A [`f64`].
    Float64(f64),

    /// A catch-all, with the type name as a [`String`].
    Other(String),

    /// A void type ([`unit`](https://doc.rust-lang.org/std/primitive.unit.html)).
    #[default]
    Void,
}

from_type!(String, String);
from_type!(bool, Bool);
from_type!(u8, Uint8);
from_type!(u16, Uint16);
from_type!(u32, Uint32);
from_type!(u64, Uint64);
from_type!(u128, Uint128);
from_type!(i8, Int8);
from_type!(i16, Int16);
from_type!(i32, Int32);
from_type!(i64, Int64);
from_type!(i128, Int128);
from_type!(f32, Float32);
from_type!(f64, Float64);

impl From<&str> for RustTypes {
    fn from(val: &str) -> Self {
        match val {
            "String" | "str" => Self::String(String::new()),
            "bool" => Self::Bool(false),
            "i8" => Self::Int8(0),
            "i16" => Self::Int16(0),
            "i32" => Self::Int32(0),
            "i64" => Self::Int64(0),
            "i128" => Self::Int128(0),
            "u8" => Self::Uint8(0),
            "u16" => Self::Uint16(0),
            "u32" => Self::Uint32(0),
            "u64" => Self::Uint64(0),
            "u128" => Self::Uint128(0),
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
            Self::Bool(_) => "Boolean".to_string(),
            Self::Uint8(_) | Self::Int8(_) => "Byte".to_string(),
            Self::Uint16(_) | Self::Int16(_) => "Short".to_string(),
            Self::Uint32(_) | Self::Int32(_) => "Integer".to_string(),
            Self::Uint64(_) | Self::Int64(_) => "Long".to_string(),
            Self::Uint128(_) | Self::Int128(_) => "java.math.BigInteger".to_string(),
            Self::Float32(_) => "Float".to_string(),
            Self::Float64(_) => "Double".to_string(),
            Self::Other(val) => val,
            Self::Void => "Void".to_string(),
        }
    }
}



/// A trait for objects that convert into [`jobject`]s.
pub trait AsJava<'a> {
    /// Convert into a [`jobject`].
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject;
}

impl<'a> AsJava<'a> for String {
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject {
        env.new_string(self).unwrap().as_raw()
    }
}

impl<'a> AsJava<'a> for &str {
    fn as_java(self, env: jni::JNIEnv<'a>) -> jni::sys::jobject {
        env.new_string(self.to_string()).unwrap().as_raw()
    }
}

macro_rules! conversion {
    ($ty: ty => $other: ident) => {
        impl<'a> AsJava<'a> for $ty {
            fn as_java(self, _env: jni::JNIEnv<'a>) -> jni::sys::jobject {
                self as jni::sys::$other as jni::sys::jobject
            }
        }
    };
}

conversion!(u8 => jbyte);
conversion!(u16 => jshort);
conversion!(u32 => jint);
conversion!(u64 => jlong);
conversion!(i8 => jbyte);
conversion!(i16 => jshort);
conversion!(i32 => jint);
conversion!(i64 => jlong);
conversion!(char => jchar);
conversion!(bool => jboolean);

// TODO
// conversion!(f32 => jfloat);
// conversion!(f64 => jdouble);
// conversion!(u128 => jbigint);
// conversion!(i128 => jbigint);


#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_example_HelloWorld_jni_1hello<'local>(
    mut env: jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, HelloWorld::hello(), "com/example/HelloWorld".to_string())
}

