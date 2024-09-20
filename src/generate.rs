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

const TYPES_CODE: &'static str = include_str!("./types.rs");
const INCLUDES_CODE: &'static str = include_str!("./include.rs");
const CONVERSIONS_CODE: &'static str = include_str!("./conv.rs");

/// Generate Rust bindings and write them to a file.
pub fn gen_code(gen: Generator, exprs: Vec<Expr>, out_file: PathBuf) -> Result<()> {
    let mut data = format!(
        "{}\n\n{}\n\n{}\n\n",
        INCLUDES_CODE, TYPES_CODE, CONVERSIONS_CODE
    )
    .split("\n")
    .filter(|v| !v.starts_with("//!"))
    .collect::<Vec<_>>()
    .join("\n");

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
