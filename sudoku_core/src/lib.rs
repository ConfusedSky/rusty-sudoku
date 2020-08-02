#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]

use ansi_term::Color;
use ansi_term::Style;

#[derive(Debug)]
pub enum ParseError {
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

pub struct Grid([[Cell; 9]; 9]);

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

    pub fn solve(&mut self) {
        println!("{}", self);

        while let Some((i, j, digit)) = self
            .find_hidden_single()
            .or_else(|| self.find_naked_single())
        {
            self.place_digit((i, j), digit);

            println!("{}", self);
        }
    }

    fn find_naked_single(&self) -> Option<(usize, usize, u8)> {
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

                        println!("Found naked single for {} at r{}c{}!", digit, i + 1, j + 1);
                        return Some((i, j, digit));
                    }
                }
            }
        }

        None
    }

    fn get_coords_in_box(box_num: usize, index: usize) -> (usize, usize) {
        let x = (box_num / 3) * 3 + index % 3;
        let y = (box_num % 3) * 3 + index / 3;
        (x, y)
    }

    fn find_hidden_single(&self) -> Option<(usize, usize, u8)> {
        fn increment_counts(
            counts: Vec<(u8, usize)>,
            candidates: [bool; 9],
            k: usize,
        ) -> Vec<(u8, usize)> {
            candidates
                .iter()
                // Map each candidate to one or zero
                .map(|candidate| if *candidate { 1 } else { 0 })
                // Zip it with the existing counts
                .zip(counts.iter())
                // Map it to a tuple of the total count and the last k value (row column or box item) where there was that candidate
                .map(|(candidate, (sum, last_value))| {
                    (
                        candidate + sum,
                        if candidate == 1 { k } else { *last_value },
                    )
                })
                .collect()
        }

        fn find_single(counts: Vec<(u8, usize)>) -> Option<(usize, usize)> {
            counts
                .iter()
                .enumerate()
                .find(|(_, (count, _))| *count == 1)
                .map(|(digit, (_, k))| (digit + 1, *k))
        }

        for i in 0..9 {
            let mut row_counts = vec![(0u8, 0usize); 9];
            let mut column_counts = vec![(0u8, 0usize); 9];
            let mut box_counts = vec![(0u8, 0usize); 9];

            for k in 0..9 {
                // Count all candidates of this type in this row
                if let Cell::Candidates(candidates) = self.0[i][k] {
                    row_counts = increment_counts(row_counts, candidates, k);
                }
                // Count all candidates of this type in this column
                if let Cell::Candidates(candidates) = self.0[k][i] {
                    column_counts = increment_counts(column_counts, candidates, k);
                }
                // Count all candidates of this type in the same box
                // Position in box
                let (x, y) = Self::get_coords_in_box(i, k);
                if let Cell::Candidates(candidates) = self.0[x][y] {
                    box_counts = increment_counts(box_counts, candidates, k);
                }
            }

            if let Some((digit, k)) = find_single(row_counts) {
                println!(
                    "Found hidden single for {} in r{} in c{}",
                    digit,
                    i + 1,
                    k + 1
                );
                return Some((i, k, digit as u8));
            }

            if let Some((digit, k)) = find_single(column_counts) {
                println!(
                    "Found hidden single for {} in c{} in r{}",
                    digit,
                    i + 1,
                    k + 1
                );
                return Some((k, i, digit as u8));
            }

            if let Some((digit, k)) = find_single(box_counts) {
                let (x, y) = Self::get_coords_in_box(i, k);
                println!(
                    "Found hidden single for {} in box {} at position r{}c{}",
                    digit,
                    i + 1,
                    x + 1,
                    y + 1
                );
                return Some((x, y, digit as u8));
            }
        }

        None
    }

    pub fn place_digit(&mut self, position: (usize, usize), digit: u8) {
        let (i, j) = position;
        self.0[i][j] = Cell::Solved(digit);
        self.remove_candidates(position, digit);
    }

    fn remove_candidate_from(&mut self, position: (usize, usize), digit: u8) {
        let (x, y) = position;
        if let Cell::Candidates(ref mut candidates) = self.0[x][y] {
            let index = (digit - 1) as usize;
            candidates[index] = false;
        }
    }

    fn remove_candidates(&mut self, position: (usize, usize), digit: u8) {
        let (i, j) = position;
        for k in 0..9 {
            // Cross out all candidates of this type in this row
            self.remove_candidate_from((i, k), digit);
            // Cross out all candidates of this type in this column
            self.remove_candidate_from((k, j), digit);
            // Cross out all candidates of this type in the same box
            // Position in box
            let x = (i / 3) * 3 + k % 3;
            let y = (j / 3) * 3 + k / 3;
            self.remove_candidate_from((x, y), digit);
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
