// src/server/routes.rs

use anyhow::Result;

pub fn serve(port: u16) -> Result<()> {
    println!("now serving on port {}", port);

    Ok(())
}
