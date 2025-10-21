//! Functions for build scripts

use std::{env, fs, path::PathBuf};

use anyhow::Result;
use glob::glob;
use regex::Regex;

use crate::{
    codegen::{cx::Generator, java::gen_java_code, rust::gen_rust_code},
    equals_throw, if_else,
    parser::classes,
};

/// The build config.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindgenConfig {
    /// The Java package to generate bindings for.
    pub package: String,

    /// The input `.rs4j` files (or globs) that get processed.
    pub files: Vec<PathBuf>,

    /// The directory where Java bindings will be written.
    pub output: PathBuf,

    /// The path of the file to generate Rust bindings in.
    pub bindings: PathBuf,

    /// Enable using JetBrains annotations?
    pub annotations: bool,

    /// Generate Kotlin code?
    pub kotlin: bool,
}

impl BindgenConfig {
    /// Create a new [`BindgenConfig`]
    pub fn new() -> BindgenConfig {
        BindgenConfig {
            package: String::new(),
            files: Vec::new(),
            output: PathBuf::new(),
            bindings: PathBuf::new(),
            annotations: false,
            kotlin: false,
        }
    }

    /// Set the Java bindings output directory.
    pub fn output<T>(mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.output = val.into();
        self
    }

    /// Set the Rust bindings output file.
    pub fn bindings<T>(mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.bindings = val.into();
        self
    }

    /// Set the package name to generate as.
    pub fn package<T>(mut self, val: T) -> Self
    where
        T: AsRef<str>,
    {
        self.package = val.as_ref().to_string();
        self
    }

    /// Alias to [`Self::package`]
    pub fn pkg<T>(self, val: T) -> Self
    where
        T: AsRef<str>,
    {
        self.package(val)
    }

    /// Add a `.rs4j` file to the config.
    pub fn file<T>(mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.files.push(val.into());
        self
    }

    /// Add a list of `.rs4j` files to the config.
    pub fn files<T>(mut self, val: Vec<T>) -> Self
    where
        T: Into<PathBuf>,
    {
        for file in val {
            self.files.push(file.into());
        }

        self
    }

    /// Add `.rs4j` files to the config via globbing for them.
    pub fn glob<T>(mut self, val: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        for file in glob(val.as_ref())? {
            if let Ok(file) = file {
                self.files.push(file);
            }
        }

        Ok(self)
    }

    /// Enable/disable JetBrains annotations.
    pub fn annotations(mut self, val: bool) -> Self {
        self.annotations = val;
        self
    }

    /// Enable/disable Kotlin codegen.
    pub fn kotlin(mut self, enable: bool) -> Self {
        self.kotlin = enable;
        self
    }

    /// Generate bindings.
    pub fn generate(self) -> Result<Self> {
        equals_throw!(
            self.files,
            Vec::<PathBuf>::new(),
            "Files array cannot be empty!"
        );
        equals_throw!(self.package, "", "Package cannot be blank!");
        equals_throw!(
            self.output,
            PathBuf::new(),
            "Output directory must be specified!"
        );
        equals_throw!(
            self.bindings,
            PathBuf::new(),
            "Output file must be specified!"
        );

        self.generate_bindings()?;
        Ok(self)
    }

    /// Copy resources.
    pub fn post_build(self) -> Result<Self> {
        equals_throw!(
            self.files,
            Vec::<PathBuf>::new(),
            "Files array cannot be empty!"
        );
        equals_throw!(self.package, "", "Package cannot be blank!");
        equals_throw!(
            self.output,
            PathBuf::new(),
            "Output directory must be specified!"
        );
        equals_throw!(
            self.bindings,
            PathBuf::new(),
            "Output file must be specified!"
        );

        self.post_build_internal()?;
        Ok(self)
    }

    fn generate_bindings(&self) -> Result<()> {
        let cx = Generator {
            package: self.package.clone(),
            with_annotations: self.annotations,
            library: env::var("CARGO_PKG_NAME")?,
            kotlin: self.kotlin,
            out_dir: self.output.join(if_else!(self.kotlin, "kotlin", "java")),
        };

        let mut exprs = Vec::new();

        for file in self.files.clone() {
            let data = fs::read_to_string(file)?;

            exprs.append(&mut classes(data.as_str())?);
        }

        for item in &mut exprs {
            item.package = cx.package.clone();
        }

        gen_rust_code(&cx, &exprs, &self.bindings)?;
        gen_java_code(&cx, &exprs)?;

        Ok(())
    }

    fn post_build_internal(&self) -> Result<()> {
        let cx = Generator {
            package: self.package.clone(),
            with_annotations: self.annotations,
            library: env::var("CARGO_PKG_NAME")?,
            kotlin: self.kotlin,
            out_dir: self.output.join(if_else!(self.kotlin, "kotlin", "java")),
        };

        let res = self.output.join("resources");

        if !res.exists() {
            fs::create_dir_all(&res)?;
        }

        let target = env::var("TARGET")?;
        let mut lib_ext = "so";

        if target.contains("windows") {
            lib_ext = "dll";
        } else if target.contains("apple-darwin") {
            lib_ext = "dylib";
        }

        let re = Regex::new("gnueabi(?:hf)?")?;
        let dir = env::var("OUT_DIR")?;
        let target_dir = PathBuf::from(dir).join("../../..").canonicalize()?;
        let out_name = format!("{}-{}.{}", cx.library, target, lib_ext);
        let out_name = re.replace(&out_name, "gnu").to_string();
        let out_path = res.join(out_name);
        let in_name = format!("lib{}.{}", cx.library.replace("-", "_"), lib_ext);
        let in_path = target_dir.join(in_name);

        fs::copy(in_path, out_path)?;

        Ok(())
    }
}
