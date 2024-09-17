use anyhow::Result;

pub fn run() -> Result<()> {
    println!("bap {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
