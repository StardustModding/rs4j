//! Shared Java code embedded in all libraries.

/// Our native binary loader.
pub const NATIVE_LOADER: &str = include_str!("NativeLoader.java");

/// @adamheinrich's NativeUtils class: https://github.com/adamheinrich/native-utils
pub const NATIVE_UTILS: &str = include_str!("NativeUtils.java");

/// Tools for native libraries
pub const NATIVE_TOOLS: &str = include_str!("NativeTools.java");
