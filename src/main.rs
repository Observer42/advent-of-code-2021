mod day01;
mod day02;
mod day03;

use std::io::Result;

fn main() -> Result<()> {
    day01::solve()?;
    day02::solve()?;
    day03::solve()?;
    Ok(())
}
