//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate audioify;
use audioify::MusicGenerator;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_insert() {
    let mut music_gen = MusicGenerator::new();
    let contents = "fn main() -> String {}".to_string();
    let filename = "my_filename.rs".to_string();
    
    let id = music_gen.add_file(filename.clone(), contents);
    music_gen.generate_file(id);

    assert!(music_gen.song_generated(id));
    music_gen.get_song(id);

    assert_eq!(filename, music_gen.get_filename(id));
}
