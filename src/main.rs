#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]

use ansi_term::Color;
use ansi_term::Style;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ParseError {
    BadLength,
    BadWidth,
}

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
    pub fn parse(content: String) -> Result<Grid, ParseError> {
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

        // Initially remove all candidates for the givens
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Solved(digit) = grid.0[i][j] {
                    grid.remove_candidates((i, j), digit);
                }
            }
        }

        return Ok(grid);
    }

    pub fn find_naked_single(&self) -> Option<(usize, usize, u8)> {
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Candidates(candidates) = self.0[i][j] {
                    if candidates.iter().filter(|x| **x).count() == 1 {
                        let digit = candidates
                            .iter()
                            .enumerate()
                            .find(|(_, b)| **b)
                            .map(|x| x.0 + 1)
                            .unwrap() as u8;
                        return Some((i, j, digit));
                    }
                }
            }
        }

        None
    }

    pub fn place_digit(&mut self, position: (usize, usize), digit: u8) {
        let (i, j) = position;
        self.0[i][j] = Cell::Solved(digit);
        self.remove_candidates(position, digit);
    }

    fn remove_candidates(&mut self, position: (usize, usize), digit: u8) {
        let (i, j) = position;
        for k in 0..9 {
            // Cross out all candidates of this type in this row
            if let Cell::Candidates(ref mut candidates) = self.0[i][k] {
                let index = (digit - 1) as usize;
                candidates[index] = false;
            }
            // Cross out all candidates of this type in this column
            if let Cell::Candidates(ref mut candidates) = self.0[k][j] {
                let index = (digit - 1) as usize;
                candidates[index] = false;
            }
            // Cross out all candidates of this type in the same box
            // Position in box
            let x = (i / 3) * 3 + k % 3;
            let y = (j / 3) * 3 + k / 3;
            if let Cell::Candidates(ref mut candidates) = self.0[x][y] {
                let index = (digit - 1) as usize;
                candidates[index] = false;
            }
        }
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

fn main() -> std::io::Result<()> {
    let filename = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("firstTest.txt"));
    let mut file = File::open(filename)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let mut grid = Grid::parse(content).unwrap();

    println!("{}", grid);

    while let Some((i, j, digit)) = grid.find_naked_single() {
        println!("Found naked single for {} at r{}c{}!", digit, i + 1, j + 1);

        grid.place_digit((i, j), digit);

        println!("{}", grid);
    }

    Ok(())
}
