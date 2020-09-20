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
    fn generate(contents: &str) -> Self {
        Self {}
    }
}

#[wasm_bindgen]
pub struct MusicGenerator {
    /// Filename_id + file map
    code: HashMap<usize, String>,
    /// Filename_id + music map
    songs: HashMap<usize, Song>,
    /// Filename_id + filename map
    files: HashMap<usize, String>,
    counter: usize,
}

#[wasm_bindgen]
impl MusicGenerator {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            code: HashMap::new(),
            files: HashMap::new(),
            counter: 0
        }
    }

    pub fn add_file(&mut self, filename: String, contents: String) -> usize {
        self.counter += 1;
        self.code.insert(self.counter, contents);
        self.files.insert(self.counter, filename);
        self.counter
    }

    pub fn generate_file(&mut self, filename: usize) {
        self.songs.insert(filename, Song::generate(&self.code[&filename]));
    }

    pub fn song_generated(&self, filename: usize) -> bool {
        self.songs.get(&filename).is_some()
    }

    pub fn get_song(&self, filename: usize) -> *const Song {
        (&self.songs[&filename]) as *const Song
    }
}
