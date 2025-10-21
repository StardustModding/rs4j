//! Shared Java code embedded in all libraries.

/// Java variants of the shared code.
pub mod java {
    /// Our native binary loader.
    pub const NATIVE_LOADER: &str = include_str!("NativeLoader.java");

    /// @adamheinrich's NativeUtils class: <https://github.com/adamheinrich/native-utils>
    pub const NATIVE_UTILS: &str = include_str!("NativeUtils.java");

    /// Tools for native libraries
    pub const NATIVE_TOOLS: &str = include_str!("NativeTools.java");

    /// The parent class
    pub const PARENT_CLASS: &str = include_str!("ParentClass.java");

    /// The native class
    pub const NATIVE_CLASS: &str = include_str!("NativeClass.java");
}

/// Kotlin variants of the shared code.
pub mod kotlin {
    /// Our native binary loader.
    pub const NATIVE_LOADER: &str = include_str!("NativeLoader.kt");

    /// @adamheinrich's NativeUtils class: <https://github.com/adamheinrich/native-utils>
    pub const NATIVE_UTILS: &str = include_str!("NativeUtils.kt");

    /// Tools for native libraries
    pub const NATIVE_TOOLS: &str = include_str!("NativeTools.kt");

    /// The parent class
    pub const PARENT_CLASS: &str = include_str!("ParentClass.kt");

    /// The native class
    pub const NATIVE_CLASS: &str = include_str!("NativeClass.kt");
}
