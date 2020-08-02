use wasm_bindgen::prelude::*;
use web_sys::console;

use sudoku_core::Grid;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn log(str: String) {
    unsafe {
        console::log_1(&JsValue::from_str(&str));
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let file = include_str!("../../sudoku_core/firstTest.txt");

    let mut grid = Grid::parse(file).unwrap();

    for step in grid.solve(|g| log(g.to_string())) {
        log(step.message);
    }

    log(grid.to_string());

    Ok(())
}
