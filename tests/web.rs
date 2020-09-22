//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate audioify;
use audioify::Song;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_insert() {
    let contents = "fn main() -> String {}".to_string();
    let filename = "my_filename.rs".to_string();
    let mut song = Song::new(filename, contents);
}
