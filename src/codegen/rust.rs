//! Rust codegen

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

use crate::{class::JavaClassBuilder, parser::expr::Expr};

use super::gen::Generator;

/// Generate Rust bindings and write them to a file.
pub fn gen_rust_code(gen: Generator, exprs: Vec<Expr>, out_file: PathBuf) -> Result<()> {
    let mut data = "use rs4j::prelude::*;\n\n".to_string();

    for expr in exprs {
        if let Expr::Class(class) = expr {
            let build = JavaClassBuilder::new(class.name.ident_strict()?, &gen.package).of(class);

            // data.push_str(&gen_class(&gen, class)?);
            data.push_str(&format!("{}\n\n", build.rust_code()));
        }
    }

    if !out_file.parent().unwrap().exists() {
        fs::create_dir_all(out_file.parent().unwrap())?;
    }

    let mut file = File::create(out_file)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}
