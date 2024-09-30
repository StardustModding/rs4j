//! Rust codegen

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

use crate::class::Class;

use super::gen::Generator;

/// Generate Rust bindings and write them to a file.
pub fn gen_rust_code(gen: Generator, classes: Vec<Class>, out_file: PathBuf) -> Result<()> {
    let mut data = "use rs4j::prelude::*;\n\n".to_string();

    for class in classes {
        let class = class.set_package(&gen.package);

        data.push_str(&format!("{}\n\n", class.rust_code()));
    }

    if !out_file.parent().unwrap().exists() {
        fs::create_dir_all(out_file.parent().unwrap())?;
    }

    let mut file = File::create(out_file)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}
