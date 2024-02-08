use std::{fs::{create_dir_all, File}, io::Write, path::PathBuf};
use anyhow::Result;
use crate::{codegen::rust::{gen_class, Generator}, parser::Expr};

pub const TYPES_CODE: &'static str = include_str!("./types.rs");
pub const INCLUDES_CODE: &'static str = include_str!("./include.rs");
pub const CONVERSIONS_CODE: &'static str = include_str!("./conv.rs");

pub fn gen_code(gen: Generator, exprs: Vec<Expr>, out_file: PathBuf) -> Result<()> {
    let mut data = format!("{}\n\n{}\n\n{}\n\n", INCLUDES_CODE, TYPES_CODE, CONVERSIONS_CODE);

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
