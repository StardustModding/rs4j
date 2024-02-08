use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use rs4j::build::BindgenConfig;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub files: Vec<PathBuf>,

    #[arg(short, long, required = true)]
    pub pkg: String,

    #[arg(short, long, required = true)]
    pub out: PathBuf,

    #[arg(short = 'f', long, required = true)]
    pub out_file: PathBuf,
}

pub fn main() -> Result<()> {
    let args = Cli::parse();

    BindgenConfig::new()
        .files(args.files)
        .package(args.pkg)
        .bindings(args.out_file)
        .output(args.out)
        .generate()?;

    Ok(())
}
