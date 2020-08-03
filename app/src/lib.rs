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
pub fn get_grid() -> JsValue {
    let file = include_str!("../../sudoku_core/firstTest.txt");

    let grid = Grid::parse(file).unwrap();

    let res = grid
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
        .collect::<Vec<Vec<Option<u8>>>>();

    return JsValue::from_serde(&res).unwrap();
}

#[wasm_bindgen]
pub fn get_solution() -> JsValue {
    let file = include_str!("../../sudoku_core/firstTest.txt");

    let mut grid = Grid::parse(file).unwrap();

    return JsValue::from_serde(&grid.solve(|_| {}).collect::<Vec<SolutionStep>>()).unwrap();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
