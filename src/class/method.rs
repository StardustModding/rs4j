//! The module for [`Method`]s.

use super::{arg::FunctionArg, ty::Type};

/// A method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Method {
    /// The name of this method.
    pub name: String,

    /// This method's arguments.
    pub args: Vec<FunctionArg>,

    /// The return [`Type`] of this method.
    pub ret: Type,

    /// Does this modify the object?
    pub is_mut: bool,

    /// Is this a static method?
    pub is_static: bool,

    /// Is this a constructor?
    pub is_init: bool,

    /// Does this return an [`Option`]?
    pub is_optional: bool,

    /// Does it consume the object?
    pub is_consumed: bool,

    /// Is there another struct this should use to call the function?
    pub object: Option<String>,

    /// Is there a custom name?
    pub custom_name: Option<String>,

    /// Does it need to be boxed?
    pub boxed: bool,
}

impl Method {
    /// Get the name of the native method this calls.
    pub fn calls(&self) -> String {
        format!(
            "jni_{}",
            self.custom_name.clone().unwrap_or(self.name.clone())
        )
    }
}
