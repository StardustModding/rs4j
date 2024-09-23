//! Rust bindings generator

use crate::{
    codegen::gen::{gen_class, Generator},
    parser::expr::Expr,
};
use anyhow::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

/// Generate Rust bindings and write them to a file.
pub fn gen_code(gen: Generator, exprs: Vec<Expr>, out_file: PathBuf) -> Result<()> {
    let mut data = "use rs4j::prelude::*;\n\n".to_string();

    for expr in exprs {
        if let Expr::Class(class) = expr {
            data.push_str(&gen_class(&gen, class)?);
        }
    }

    if !out_file.parent().unwrap().exists() {
        create_dir_all(out_file.parent().unwrap())?;
    }

    let mut file = File::create(out_file)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}
