//! The module for [`JavaClassBuilder`]s.

use field::Field;
use native::NativeMethod;
use wrapper::WrapperMethod;

pub mod ty;
pub mod native;
pub mod wrapper;
pub mod field;
pub mod arg;

/// A builder for a Java class.
pub struct JavaClassBuilder {
    /// The name of this class.
    pub name: String,

    /// A list of imports.
    pub imports: Vec<String>,

    /// A list of fields.
    pub fields: Vec<Field>,

    /// A list of native methods.
    pub native_methods: Vec<NativeMethod>,

    /// A list of wrapper methods.
    pub wrapper_methods: Vec<WrapperMethod>,
}

impl JavaClassBuilder {
    /// Create a new [`JavaClassBuilder`].
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().into(),
            imports: Vec::new(),
            fields: Vec::new(),
            native_methods: Vec::new(),
            wrapper_methods: Vec::new(),
        }
    }
}
