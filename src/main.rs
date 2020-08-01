#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]

use ansi_term::Color;
use ansi_term::Style;
use std::fs::File;
use std::io::prelude::*;

enum CellPart {
    Top,
    Middle,
    Bottom,
}

impl CellPart {
    fn range(&self) -> std::ops::Range<usize> {
        match self {
            CellPart::Top => (1..4),
            CellPart::Middle => (4..7),
            CellPart::Bottom => (7..10),
        }
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Solved(u8),
    Candidates([bool; 9]),
}

impl Cell {
    fn draw_part(
        &self,
        f: &mut std::fmt::Formatter,
        cell_part: CellPart,
        position: (usize, usize),
    ) -> std::fmt::Result {
        let normal = if (position.0 + position.1) % 2 == 0 {
            Style::new().on(Color::RGB(50, 50, 50))
        } else {
            Style::new()
        };
        let bold = normal.bold().fg(Color::Red);

        match self {
            Cell::Solved(num) => {
                if let CellPart::Middle = cell_part {
                    write!(f, "{}", bold.paint(format!(" {} ", num)))?;
                } else {
                    write!(f, "{}", bold.paint(format!("   ")))?;
                }
            }
            Cell::Candidates(c) => {
                let candidate_string = cell_part
                    .range()
                    .map(|x| {
                        if !c[x - 1] {
                            String::from(" ")
                        } else {
                            x.to_string()
                        }
                    })
                    .collect::<String>();

                write!(f, "{}", normal.paint(candidate_string))?
            }
        }

        Ok(())
    }
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

        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Solved(digit) = grid.0[i][j] {
                    for k in 0..9 {
                        // Cross out all candidates of this type in this row
                        if let Cell::Candidates(ref mut candidates) = grid.0[i][k] {
                            let index = (digit - 1) as usize;
                            candidates[index] = false;
                        }
                        // Cross out all candidates of this type in this column
                        if let Cell::Candidates(ref mut candidates) = grid.0[k][j] {
                            let index = (digit - 1) as usize;
                            candidates[index] = false;
                        }
                        // Cross out all candidates of this type in the same box
                        // Position in box
                        let x = (i / 3) * 3 + k % 3;
                        let y = (j / 3) * 3 + k / 3;
                        if let Cell::Candidates(ref mut candidates) = grid.0[x][y] {
                            let index = (digit - 1) as usize;
                            candidates[index] = false;
                        }
                    }
                }
            }
        }

        return Ok(grid);
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Each cell is 4 wide and there are two dividers per line
        let width = 3 * 9 + 2;

        for (i, line) in self.0.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                let dashes = (0..width).map(|_| "-").collect::<String>();
                writeln!(f, "{}", dashes)?;
            }

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, CellPart::Top, (i, j))?;
            }

            writeln!(f, "")?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, CellPart::Middle, (i, j))?;
            }

            writeln!(f, "")?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, CellPart::Bottom, (i, j))?;
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
