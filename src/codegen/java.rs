//! Java codegen.

use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::{
    class::Class,
    java::{NATIVE_CLASS, NATIVE_TOOLS, NATIVE_UTILS, PARENT_CLASS},
    loader::generate_loader,
};

use super::gen::Generator;

/// Generate the Java code for an entire `.rs4j` file.
pub fn gen_java_code(gen: Generator, classes: Vec<Class>, out: PathBuf) -> Result<()> {
    for class in classes {
        let class = class.set_package(&gen.package);
        let dir = out.join("java/src").join(gen.dir_pkg());
        let path = dir.join(format!("{}.java", class.name));
        let code = class.java_code();

        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        fs::write(path, code)?;
    }

    let dir = out.join("java/src").join(gen.dir_pkg());
    let path = dir.join("NativeLoader.java");

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    fs::write(path, generate_loader(gen.package, gen.library))?;

    let dir = out.join("java/src/cz/adamh/utils");
    let path = dir.join("NativeUtils.java");

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    fs::write(path, NATIVE_UTILS)?;

    let dir = out.join("java/src/org/stardustmodding/rs4j/util");
    let path = dir.join("NativeTools.java");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    fs::write(path, NATIVE_TOOLS)?;

    let path = dir.join("ParentClass.java");

    fs::write(path, PARENT_CLASS)?;

    let path = dir.join("NativeClass.java");

    fs::write(path, NATIVE_CLASS)?;

    Ok(())
}
