use anyhow::Result;
use rs4j::build::BindgenConfig;

fn main() -> Result<()> {
    let src_path = format!("{}/java/src/generated", env!("CARGO_MANIFEST_DIR"));

    BindgenConfig::new()
        .package("com.example")
        .bindings(format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))
        .glob(format!("{}/bindings/**/*.rs4j", env!("CARGO_MANIFEST_DIR")))?
        .output(&src_path)
        .annotations(false)
        .kotlin(true)
        .generate()?;

    Ok(())
}
