use anyhow::Result;
use rs4j::build::BindgenConfig;

fn main() -> Result<()> {
    let out_path = format!("{}/generated", env!("CARGO_MANIFEST_DIR"));

    BindgenConfig::new()
        .package("com.example")
        .bindings(format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))
        .glob(format!("{}/bindings/**/*.rs4j", env!("CARGO_MANIFEST_DIR")))?
        .output(&out_path)
        .annotations(false)
        .generate()?;

    Ok(())
}
