mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Song {}

#[wasm_bindgen]
impl Song {
    fn generate(contents: String) -> Self {
        Self {}
    }
}

#[wasm_bindgen]
pub struct MusicGenerator {
    /// Filename + file map
    code: HashMap<String, String>,
    /// Filename + music map
    songs: HashMap<String, Song>,
}

#[wasm_bindgen]
impl MusicGenerator {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            code: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, filename: String, contents: String) {
        self.code.insert(filename, contents);
    }

    pub fn generate_file(&mut self, filename: String, contents: String) {
        self.songs.insert(filename, Song::generate(contents));
    }

    pub fn song_generated(&self, filename: &str) -> bool {
        self.songs.get(filename).is_some()
    }

    pub fn get_song(&self, filename: &str) -> *const Song {
        self.songs.get(filename).unwrap() as *const Song
    }
}
