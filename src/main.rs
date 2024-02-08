use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use rs4j::{gen_code, java::gen_java_code, parser::classes, Generator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub files: Vec<String>,

    #[arg(short, long, required = true)]
    pub pkg: String,

    #[arg(short, long, required = true)]
    pub out: PathBuf,

    #[arg(short = 'f', long, required = true)]
    pub out_file: PathBuf,
}

pub fn main() -> Result<()> {
    let args = Cli::parse();
    let gen = Generator { package: args.pkg };
    let mut exprs = Vec::new();

    for file in args.files {
        let data = fs::read_to_string(file)?;

        exprs.append(&mut classes(data.as_str())?);
    }

    gen_code(gen.clone(), exprs.clone(), args.out_file)?;
    gen_java_code(gen, exprs, args.out)?;

    Ok(())
}
