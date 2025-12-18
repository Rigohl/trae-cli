use anyhow::Result;
use cargo_metadata::MetadataCommand;
#[doc = "Function documentation added by AI refactor"]
pub fn estimate_cargo_units() -> usize {
    estimate_cargo_units_inner().unwrap_or(100)
}
#[doc = "Function documentation added by AI refactor"]
fn estimate_cargo_units_inner() -> Result<usize> {
    let metadata = MetadataCommand::new().no_deps().exec()?;
    if let Some(resolve) = metadata.resolve {
        if !resolve.nodes.is_empty() {
            return Ok(resolve.nodes.len());
        }
    }
    Ok(metadata.packages.len().max(1))
}
