use std::env;
use std::error::Error;
use std::fs::File;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -- <file_path>")?;
    let _file = File::open(&path)?;

    Ok(())
}
