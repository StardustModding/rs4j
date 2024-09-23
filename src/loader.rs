//! Codegen for the natives loader.

const CODE: &str = include_str!("NativeLoader.java");

/// @adamheinrich's NativeUtils class: https://github.com/adamheinrich/native-utils
pub const NATIVE_UTILS: &str = include_str!("NativeUtils.java");

/// Generate a NativeLoader class.
pub fn generate_loader(package: impl AsRef<str>, lib_name: impl AsRef<str>) -> String {
    CODE.replace("$package$", package.as_ref())
        .replace("$library$", lib_name.as_ref())
}
