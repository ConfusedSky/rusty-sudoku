use ansi_term::Style;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Cell {
    Solved(u8),
    Candidates([bool; 9]),
}

struct Grid([[Cell; 9]; 9]);

impl Default for Grid {
    fn default() -> Self {
        Grid([[Cell::Candidates([true; 9]); 9]; 9])
    }
}

impl Grid {
    fn parse(content: String) -> Result<Grid, ParseError> {
        let cleaned = content
            .split("\n")
            .filter(|x| !x.contains("-"))
            .map(|x| x.replace("|", ""))
            .collect::<Vec<String>>();

        // Make sure we got a 9x9 grid
        if cleaned.len() != 9 {
            return Err(ParseError::BadLength);
        }

        if cleaned.iter().any(|x| x.len() != 9) {
            return Err(ParseError::BadWidth);
        }

        let mut grid: Grid = Default::default();

        for (i, line) in cleaned
            .iter()
            .map(|x| x.chars().map(|x| x.to_digit(10).map(|x| x as u8)))
            .enumerate()
        {
            for (j, digit) in line.enumerate() {
                if let Some(digit) = digit {
                    grid.0[i][j] = Cell::Solved(digit);
                }
            }
        }

        return Ok(grid);
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Each cell is 3 wide and there are two dividers per line
        let width = 3 * 9 + 2;
        let bold = Style::new().bold();

        for (i, line) in self.0.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                let dashes = (0..width).map(|_| "-").collect::<String>();
                writeln!(f, "{}", dashes)?;
            }

            let separators = (0..3)
                .map(|_| String::from("         "))
                .collect::<Vec<String>>()
                .join("|");

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                match digit {
                    Cell::Solved(_) => write!(f, "   ")?,
                    Cell::Candidates(_) => write!(f, "   ")?,
                }
            }

            writeln!(f, "")?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                match digit {
                    Cell::Solved(num) => write!(f, " {} ", bold.paint(num.to_string()))?,
                    Cell::Candidates(_) => write!(f, "   ")?,
                }
            }

            writeln!(f, "")?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                match digit {
                    Cell::Solved(_) => write!(f, "   ")?,
                    Cell::Candidates(_) => write!(f, "   ")?,
                }
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum ParseError {
    BadLength,
    BadWidth,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("firstTest.txt")?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let grid = Grid::parse(content).unwrap();

    println!("{}", grid);

    Ok(())
}
