use std::{fs, path::PathBuf};

use anyhow::Result;
use glob::glob;

use crate::{
    codegen::{gen::Generator, java::gen_java_code},
    equals_throw,
    generate::gen_code,
    parser::parser::classes,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindgenConfig {
    /// The Java package to generate bindings for.
    pub package: String,

    /// The input `.rs4j` files (or globs) that get processed.
    pub files: Vec<PathBuf>,

    /// The directory where Java bindings.
    pub output: PathBuf,

    /// The path of the file to generate Rust bindings in.
    pub bindings: PathBuf,

    /// Enable using JetBrains annotations?
    pub annotations: bool,
}

impl BindgenConfig {
    pub fn new() -> BindgenConfig {
        BindgenConfig {
            package: String::new(),
            files: Vec::new(),
            output: PathBuf::new(),
            bindings: PathBuf::new(),
            annotations: false,
        }
    }

    pub fn output<T>(&mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.output = val.into();
        self.clone()
    }

    pub fn bindings<T>(&mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.bindings = val.into();
        self.clone()
    }

    pub fn package<T>(&mut self, val: T) -> Self
    where
        T: AsRef<str>,
    {
        self.package = val.as_ref().to_string();
        self.clone()
    }

    pub fn pkg<T>(&mut self, val: T) -> Self
    where
        T: AsRef<str>,
    {
        self.package(val)
    }

    pub fn file<T>(&mut self, val: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.files.push(val.into());
        self.clone()
    }

    pub fn files<T>(&mut self, val: Vec<T>) -> Self
    where
        T: Into<PathBuf>,
    {
        for file in val {
            self.files.push(file.into());
        }

        self.clone()
    }

    pub fn glob<T>(&mut self, val: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        for file in glob(val.as_ref())? {
            if let Ok(file) = file {
                self.files.push(file);
            }
        }

        Ok(self.clone())
    }

    pub fn annotations(&mut self, val: bool) -> Self {
        self.annotations = val;
        self.clone()
    }

    pub fn generate(&self) -> Result<()> {
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

        self.generate_bindings()
    }

    fn generate_bindings(&self) -> Result<()> {
        let gen = Generator {
            package: self.package.clone(),
            with_annotations: self.annotations,
        };

        let mut exprs = Vec::new();

        for file in self.files.clone() {
            let data = fs::read_to_string(file)?;

            exprs.append(&mut classes(data.as_str())?);
        }

        gen_code(gen.clone(), exprs.clone(), self.bindings.clone())?;
        gen_java_code(gen, exprs, self.output.clone())?;

        Ok(())
    }
}
