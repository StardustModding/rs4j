#[allow(unused_imports)]
use jni::sys::{
    jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort, jstring,
    jvalue,
};

use jni::{
    objects::{JObject, JValueGen},
    JNIEnv,
};

#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

#[allow(dead_code)]
fn object_to_jobject<T>(env: *mut JNIEnv, obj: T, jcls: String) -> jobject {
    let jobj: JObject = unsafe { (*env).alloc_object(jcls).unwrap() };

    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");

    let ret: jlong = Box::into_raw(Box::new(obj)) as jlong;

    unsafe {
        let res = (*env).set_field(&jobj, "__pointer", "Long", JValueGen::Long(ret));

        if (*env).exception_check().unwrap() || res.is_err() {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }

    jobj.as_raw()
}


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
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum RustTypes {
    String(String),
    Bool(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    Other(String),

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_demo_Duration_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    secs: u64,
    nanos: u32
) -> jobject {
    object_to_jobject(env, Duration::new(secs, nanos), "org/stardustmodding/rs4j/demo/Duration".to_string())
}

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
pub unsafe extern "system" fn Java_org_stardustmodding_rs4j_demo_Duration_jni_as_secs<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Duration = jlong_to_pointer::<Duration>(this).as_mut().unwrap();
    object_to_jobject(env, Duration::as_secs(this), "org/stardustmodding/rs4j/demo/Duration".to_string())
}

