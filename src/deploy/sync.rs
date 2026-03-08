// src/deploy/sync.rs

use anyhow::Result;

pub fn transfer(target: Option<String>) -> Result<()> {
    println!("Deploy target={:?}", target);

    Ok(())
}
