mod day01;
mod day02;

use std::io::Result;

fn main() -> Result<()> {
    day01::solve()?;
    day02::solve()?;
    Ok(())
}
