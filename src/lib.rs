#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[macro_use]
extern crate peg;

#[macro_use]
pub extern crate anyhow;

#[cfg(feature = "build")]
pub mod build;

pub mod base;
pub mod codegen;
pub mod conv;
pub mod generate;
pub mod include;
pub mod loader;
pub mod macros;
pub mod parser;
pub mod types;

pub mod prelude {
    //! Base types.

    pub use super::base::*;
    pub use super::conv::*;
    pub use super::include::*;
    pub use super::types::*;
    pub use jni::sys::{
        jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort,
        jstring, jvalue,
    };
    pub use jni::*;
}
