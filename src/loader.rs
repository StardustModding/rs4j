//! Codegen for the natives loader.

use crate::{
    codegen::cx::Generator,
    java::{java, kotlin},
};

/// Generate a NativeLoader class.
pub fn generate_loader(cx: &Generator) -> String {
    if cx.kotlin {
        kotlin::NATIVE_LOADER
    } else {
        java::NATIVE_LOADER
    }
    .replace("$package$", &cx.package)
    .replace("$library$", &cx.library)
}
