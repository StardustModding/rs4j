#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[macro_use]
extern crate peg;

#[macro_use]
pub extern crate anyhow;

#[cfg(feature = "build")]
pub mod build;

pub mod codegen;
pub mod generate;
pub mod loader;
pub mod macros;
pub mod parser;
pub mod internal;
pub mod java;
pub mod class;

pub mod prelude {
    //! Base types.

    pub use super::internal::base::*;
    pub use super::internal::conv::*;
    pub use super::internal::include::*;
    pub use super::internal::types::*;
    pub use jni::sys::{
        jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort,
        jstring, jvalue,
    };
    pub use jni::*;
}
