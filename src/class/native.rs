//! Native methods.

use std::collections::HashMap;
use super::ty::Type;

/// A native method.
/// This has no return as native methods will always return a pointer (long/u64).
pub struct NativeMethod {
    /// The name of this method.
    pub name: String,

    /// This method's args.
    pub args: HashMap<String, Type>,

    /// Is this method static?
    pub is_static: bool,
}

impl NativeMethod {
    /// Generate Java code for this method.
    pub fn java_code(&self) -> String {
        todo!("How to handle custom types? Potentially make a transformer on both ends?")
    }

    /// Generate Rust code for this method.
    pub fn rust_code(&self) -> String {
        todo!("How to handle custom types? Potentially make a transformer on both ends?")
    }
}
