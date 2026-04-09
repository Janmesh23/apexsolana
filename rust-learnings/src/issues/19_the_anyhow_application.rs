// ============================================================
// ISSUE 19 — The Anyhow Application
// Concept: Application-level Error Handling
// Difficulty: Intermediate
//
// Your Task:
// While `thiserror` is great for libraries, `anyhow` is often preferred
// for applications because it allows for easy error propagation and
// adding human-readable context.
//
// TODO: Refactor `main` to return `anyhow::Result<()>`.
// TODO: Use the `.context()` or `.with_context()` methods to add meaningful
// information to potential errors.
//
// Resources: https://docs.rs/anyhow/latest/anyhow/
// ============================================================

use anyhow::{Context, Result};
use std::fs;

pub fn read_config() -> Result<String> {
    // TODO: Fix this line to use context
    let content = fs::read_to_string("config.toml")?;
    Ok(content)
}

pub fn parse_config(content: &str) -> Result<u32> {
    // TODO: Fix this line to use context
    let val = content.trim().parse::<u32>()?;
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyhow_context() {
        // Since config.toml likely doesn't exist, we expect an error with context.
        let res = read_config();
        assert!(res.is_err());
        // println!("Error logic: {:?}", res.err());
    }
}
