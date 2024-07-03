use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let file = fs::read_to_string("README.md")?;

    print!("{file}");

    Ok(())
}
