#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]

use serde::{Deserialize, Serialize};

use ansi_term::Color;
use ansi_term::Style;

#[derive(Debug)]
pub enum ParseError {
    BadLength,
    BadWidth,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionStep {
    pub position: (usize, usize),
    pub digit: u8,
    pub message: String,
}

enum CellPart {
    Top,
    Middle,
    Bottom,
}

impl CellPart {
    fn range(&self) -> std::ops::Range<usize> {
        match self {
            Self::Top => (1..4),
            Self::Middle => (4..7),
            Self::Bottom => (7..10),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Solved(u8),
    Candidates([bool; 9]),
}

impl Cell {
    fn draw_part(
        &self,
        f: &mut std::fmt::Formatter,
        cell_part: &CellPart,
        position: (usize, usize),
        terminal: bool,
    ) -> std::fmt::Result {
        let normal = if (position.0 + position.1) % 2 == 0 {
            Style::new().on(Color::RGB(50, 50, 50))
        } else {
            Style::new()
        };
        let bold = normal.bold().fg(Color::Red);

        let mut draw = |text: String, style: Style| -> std::fmt::Result {
            if terminal {
                write!(f, "{}", style.paint(text))
            } else {
                write!(f, "{}", text)
            }
        };

        match self {
            Self::Solved(num) => {
                if let CellPart::Middle = cell_part {
                    draw(format!(" {} ", num), bold)?;
                } else {
                    draw("   ".to_string(), bold)?;
                }
            }
            Self::Candidates(c) => {
                let candidate_string = cell_part
                    .range()
                    .map(|x| {
                        if c[x - 1] {
                            x.to_string()
                        } else {
                            String::from(" ")
                        }
                    })
                    .collect::<String>();

                draw(candidate_string, normal)?;
            }
        }

        Ok(())
    }
}

pub struct Grid {
    grid: [[Cell; 9]; 9],
    printtty: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            grid: [[Cell::Candidates([true; 9]); 9]; 9],
            printtty: false,
        }
    }
}

impl Grid {
    /// # Errors
    /// Will return an error if the input doesn't have the correct number
    /// of rows of columns
    pub fn parse<S: Into<String>>(content: S) -> Result<Self, ParseError> {
        let st = content.into();

        let cleaned = st
            .split('\n')
            .filter_map(|x| {
                if x.contains('-') {
                    None
                } else {
                    Some(x.replace("|", ""))
                }
            })
            .collect::<Vec<String>>();

        // Make sure we got a 9x9 grid
        if cleaned.len() != 9 {
            return Err(ParseError::BadLength);
        }

        if cleaned.iter().any(|x| x.len() != 9) {
            return Err(ParseError::BadWidth);
        }

        let mut grid = Self::default();

        #[allow(clippy::cast_possible_truncation)]
        for (i, line) in cleaned
            .iter()
            .map(|x| x.chars().map(|x| x.to_digit(10).map(|x| x as u8)))
            .enumerate()
        {
            for (j, digit) in line.enumerate() {
                if let Some(digit) = digit {
                    grid.grid[i][j] = Cell::Solved(digit);
                }
            }
        }

        // Initially remove all candidates for the givens
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Solved(digit) = grid.grid[i][j] {
                    grid.remove_candidates((i, j), digit);
                }
            }
        }

        Ok(grid)
    }

    pub fn settty(&mut self, bool: bool) {
        self.printtty = bool;
    }

    #[must_use]
    pub const fn get_grid(&self) -> &[[Cell; 9]; 9] {
        &self.grid
    }

    pub fn solve<F>(&mut self, before_step: F) -> GridIterator<F>
    where
        F: Fn(&Self)
    {
        GridIterator {
            grid: self,
            before_step,
        }
    }

    fn find_naked_single(&self) -> Option<SolutionStep> {
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Candidates(candidates) = self.grid[i][j] {
                    if candidates.iter().filter(|x| **x).count() == 1 {
                        // Find the digit that has at least one canditate
                        // or return nothing
                        #[allow(clippy::cast_possible_truncation)]
                        let digit = candidates
                            .iter()
                            .enumerate()
                            .find_map(|(x, b)| {
                                if *b {
                                    Some(x + 1)
                                } else {
                                    None
                                }
                            })
                            .unwrap() as u8;

                        return Some(SolutionStep {
                            position: (i, j),
                            digit,
                            message: format!(
                                "Found naked single for {} at r{}c{}!",
                                digit,
                                i + 1,
                                j + 1
                            ),
                        });
                    }
                }
            }
        }

        None
    }

    const fn get_coords_in_box(box_num: usize, index: usize) -> (usize, usize) {
        let x = (box_num / 3) * 3 + index % 3;
        let y = (box_num % 3) * 3 + index / 3;
        (x, y)
    }

    fn find_hidden_single(&self) -> Option<SolutionStep> {
        fn increment_counts(
            counts: &[(u8, usize)],
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

        fn find_single(counts: &[(u8, usize)]) -> Option<(usize, usize)> {
            counts
                .iter()
                .enumerate()
                .find_map(|(digit, (count, k))| {
                    if *count == 1 {
                        Some((digit + 1, *k))
                    } else {
                        None
                    }
                })
        }

        for i in 0..9 {
            let mut row_counts = vec![(0_u8, 0_usize); 9];
            let mut column_counts = vec![(0_u8, 0_usize); 9];
            let mut box_counts = vec![(0_u8, 0_usize); 9];

            for k in 0..9 {
                // Count all candidates of this type in this row
                if let Cell::Candidates(candidates) = self.grid[i][k] {
                    row_counts = increment_counts(&row_counts, candidates, k);
                }
                // Count all candidates of this type in this column
                if let Cell::Candidates(candidates) = self.grid[k][i] {
                    column_counts = increment_counts(&column_counts, candidates, k);
                }
                // Count all candidates of this type in the same box
                // Position in box
                let (x, y) = Self::get_coords_in_box(i, k);
                if let Cell::Candidates(candidates) = self.grid[x][y] {
                    box_counts = increment_counts(&box_counts, candidates, k);
                }
            }

            if let Some((digit, k)) = find_single(&box_counts) {
                let (x, y) = Self::get_coords_in_box(i, k);
                let message = format!(
                    "Found hidden single for {} in box {} at position r{}c{}",
                    digit,
                    i + 1,
                    x + 1,
                    y + 1
                );

                #[allow(clippy::cast_possible_truncation)]
                return Some(SolutionStep {
                    position: (x, y),
                    digit: digit as u8,
                    message,
                });
            }

            if let Some((digit, k)) = find_single(&row_counts) {
                let message = format!(
                    "Found hidden single for {} in r{} at c{}",
                    digit,
                    i + 1,
                    k + 1
                );

                #[allow(clippy::cast_possible_truncation)]
                return Some(SolutionStep {
                    position: (i, k),
                    digit: digit as u8,
                    message,
                });
            }

            if let Some((digit, k)) = find_single(&column_counts) {
                let message = format!(
                    "Found hidden single for {} in c{} at r{}",
                    digit,
                    i + 1,
                    k + 1
                );

                #[allow(clippy::cast_possible_truncation)]
                return Some(SolutionStep {
                    position: (k, i),
                    digit: digit as u8,
                    message,
                });
            }
        }

        None
    }

    pub fn place_digit(&mut self, position: (usize, usize), digit: u8) {
        let (i, j) = position;
        self.grid[i][j] = Cell::Solved(digit);
        self.remove_candidates(position, digit);
    }

    fn remove_candidate_from(&mut self, position: (usize, usize), digit: u8) {
        let (x, y) = position;
        if let Cell::Candidates(ref mut candidates) = self.grid[x][y] {
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

        for (i, line) in self.grid.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                let dashes = (0..width).map(|_| "-").collect::<String>();
                writeln!(f, "{}", dashes)?;
            }

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, &CellPart::Top, (i, j), self.printtty)?;
            }

            writeln!(f)?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, &CellPart::Middle, (i, j), self.printtty)?;
            }

            writeln!(f)?;

            for (j, digit) in line.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "|")?;
                }

                digit.draw_part(f, &CellPart::Bottom, (i, j), self.printtty)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct GridIterator<'s, F>
where
    F: Fn(&Grid),
{
    grid: &'s mut Grid,
    before_step: F,
}

impl<'s, F> Iterator for GridIterator<'s, F>
where
    F: Fn(&Grid),
{
    type Item = SolutionStep;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .grid
            .find_hidden_single()
            .or_else(|| self.grid.find_naked_single());

        if let Some(ref step) = next {
            let before_step = &mut self.before_step;
            before_step(self.grid);
            self.grid.place_digit(step.position, step.digit);
        }

        next
    }
}
