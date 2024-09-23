use anyhow::Result;
use clap::{Parser, Subcommand};
use rs4j::build::BindgenConfig;
use serde_json::Value;
use std::{env, fs, path::PathBuf, process::Command};

const CURRENT: &str = env!("TARGET");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Compile {
        files: Vec<PathBuf>,

        #[arg(short, long, required = true)]
        pkg: String,

        #[arg(short, long, required = true)]
        out: PathBuf,

        #[arg(short = 'f', long, required = true)]
        out_file: PathBuf,
    },

    Build {
        args: Vec<String>,

        #[arg(short, long)]
        zigbuild: bool,
    },
}

pub fn run_build(args: Vec<String>, zigbuild: bool) -> Result<()> {
    let build_cmd = if zigbuild { "zigbuild" } else { "build" };

    Command::new("cargo")
        .envs(env::vars())
        .arg(build_cmd)
        .args(&args)
        .spawn()?
        .wait()?;

    if !fs::exists("post-build.rs")? {
        return Ok(());
    }

    Command::new("cargo")
        .envs(env::vars())
        .arg("build")
        .arg("--features")
        .arg("post-build")
        .arg("--bin")
        .arg("post-build")
        .spawn()?
        .wait()?;

    let raw = String::from_utf8(
        Command::new("cargo")
            .envs(env::vars())
            .arg("metadata")
            .arg("--no-deps")
            .output()?
            .stdout,
    )?;

    let raw_toml = String::from_utf8(
        Command::new("cargo")
            .envs(env::vars())
            .arg("read-manifest")
            .output()?
            .stdout,
    )?;

    let data = serde_json::from_str::<Value>(&raw)?;
    let data_toml = serde_json::from_str::<Value>(&raw_toml)?;

    let dir = PathBuf::from(data.get("target_directory").unwrap().as_str().unwrap()).join("debug");

    let name = data_toml.get("name").unwrap().as_str().unwrap();

    let mut target = CURRENT.to_string();
    let mut iter = args.iter();

    while let Some(it) = iter.next() {
        if it.to_lowercase() == "--target" {
            if let Some(it) = iter.next() {
                target = it.to_owned();
                break;
            }
        }
    }

    let cwd = env::current_dir()?;
    let exe = dir.join("post-build");
    let fake_dir = dir.join("tmp/fake/dir");

    if !fake_dir.exists() {
        fs::create_dir_all(&fake_dir)?;
    }

    Command::new(exe)
        .envs(env::vars())
        .env("CARGO_MANIFEST_DIR", cwd)
        .env("CARGO_PKG_NAME", name)
        .env("TARGET", target)
        .env("OUT_DIR", fake_dir)
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Compile {
            files,
            pkg,
            out_file,
            out,
        } => {
            BindgenConfig::new()
                .files(files)
                .package(pkg)
                .bindings(out_file)
                .output(out)
                .generate()?;
        }

        Commands::Build { args, zigbuild } => {
            run_build(args, zigbuild)?;
        }
    }

    Ok(())
}
