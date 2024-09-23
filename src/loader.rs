//! Codegen for the natives loader.

use crate::java::NATIVE_LOADER;

/// Generate a NativeLoader class.
pub fn generate_loader(package: impl AsRef<str>, lib_name: impl AsRef<str>) -> String {
    NATIVE_LOADER.replace("$package$", package.as_ref())
        .replace("$library$", lib_name.as_ref())
}
