use wasm_bindgen::prelude::*;
use crate::notes::{KeySignature, Key};

#[wasm_bindgen]
pub struct Song {
    filename: String,
    filename_sum: usize,
    contents: String,

    // In the format of [pitch, duration, pitch, duration]
    generated_song: Option<Box<[u16]>>,

    current_key: Option<Key>,
    key_signature: KeySignature,
    key_sig_delta: u8,
    flag: bool,
}

#[wasm_bindgen]
impl Song {
    pub fn new(filename: String, contents: String) -> Self {
        let filename_sum = filename.chars().map(|c| c as usize).sum();
        Self {
            filename_sum,
            filename,
            contents,
            generated_song: None,

            current_key: None,
            key_signature: *Self::rand_choice(
                filename_sum,
                &[KeySignature::Major, KeySignature::Minor],
            ),
            key_sig_delta: *Self::rand_choice(
                filename_sum,
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            ),
            flag: *Self::rand_choice(filename_sum, &[true, false]),
        }
    }

    // Seems "random", but will be the same choice for each filename, which is good!
    // The goal here is to make the same file ALWAYS output the same results.
    fn rand_choice<'a, T>(seed: usize, choices: &'a [T]) -> &'a T {
        &choices[seed % choices.len()]
    }

    fn get_next_note(&self, key: Key) -> Key {
        match self.current_key {
            Some(k) => k.process(&key, |x, y| {
                let delta = Self::rand_choice(y as usize, &[1, 1, 1, 1, 1, 2, 2, 3]);
                if self.flag {
                    x + delta
                } else {
                    x - delta
                }
            }),
            None => key,
        }
    }

    pub fn generate(&mut self) -> *const u16 {
        let mut notes = Vec::with_capacity(self.contents.len());
        let mut current_sum: usize = 0;

        for c in self.contents.chars() {
            if is_paren(c) {
                // Change direction of flow
                let negated = !self.flag;
                self.flag = *Self::rand_choice(
                    current_sum,
                    &[
                        negated, negated, negated, negated, negated, negated, self.flag,
                    ],
                );
            }
            current_sum += c as usize;

            // Pitch
            notes.push(self.get_next_note(Key::new(c as u8)).to_pitch());
            // Duration
            notes.push(1);
        }

        self.generated_song = Some(notes.into_boxed_slice());
        self.generated_song.as_ref().unwrap().as_ptr()
    }
}

fn is_paren(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => true,
        _ => false,
    }
}
