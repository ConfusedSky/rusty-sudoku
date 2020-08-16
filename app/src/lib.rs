#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

use sudoku_core::Cell;
use sudoku_core::Grid;
use sudoku_core::SolutionStep;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Context {
    grid: Grid,
}

pub type OptionalGrid = Vec<Vec<Option<u8>>>;

#[derive(Serialize, Deserialize)]
pub struct Solution {
    steps: Vec<SolutionStep>,
    grids: Vec<OptionalGrid>,
}

#[wasm_bindgen]
#[must_use]
pub fn default_context() -> Context {
    let file = include_str!("../../sudoku_core/firstTest.txt");

    Context { grid: Grid::parse(file).unwrap() }
}

fn grid_to_optional(grid: &Grid) -> OptionalGrid {
    grid
        .get_grid()
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| {
                    if let Cell::Solved(digit) = c {
                        Some(*digit)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<u8>>>()
        })
        .collect()
}

#[wasm_bindgen]
#[must_use]
pub fn get_solution(con: &mut Context) -> JsValue {
    let mut grids: Vec<OptionalGrid> = Vec::new();

    grids.push(grid_to_optional(&con.grid));

    let steps = con.grid.solve(|grid| {
        let converted = grid_to_optional(grid);
        grids.push(converted);
    }).collect::<Vec<SolutionStep>>();

    let solution = Solution {
        steps,
        grids
    };

    JsValue::from_serde(&solution).unwrap()
}

// This is like the `main` function, except for JavaScript.
/// # Errors
/// Doesn't actually ever throw errors
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
