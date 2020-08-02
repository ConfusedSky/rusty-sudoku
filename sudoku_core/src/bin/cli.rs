use std::fs::File;
use std::io::prelude::*;

use sudoku_core::Grid;

fn main() -> std::io::Result<()> {
    let filename = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("firstTest.txt"));
    let mut file = File::open(filename)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let mut grid = Grid::parse(content).unwrap();
    grid.settty(atty::is(atty::Stream::Stdout));

    for step in grid.solve(|g| println!("{}", g)) {
        println!("{}", step.message);
    }

    println!("{}", grid);

    Ok(())
}
