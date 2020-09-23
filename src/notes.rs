// This is helpful https://en.wikipedia.org/wiki/Piano_key_frequencies#List

/// Pitch in hertz
type Pitch = u16;

/// A piano key
#[derive(Clone, Copy)]
pub struct Key(pub u8);

impl Key {
    pub fn new(val: u8) -> Self {
        Self(val)
    }

    pub fn to_pitch(&self) -> Pitch {
        (2.0f32.powf((self.0 as f32 - 49f32) / 12f32) * 440f32) as Pitch
    }
}

#[derive(Clone, Copy)]
pub enum KeySignature {
    Major,
    Minor,
}

fn get_closest<F: Fn(&u8, &u8) -> bool>(haystack: &[u8], needle: u8, prec: F) -> &u8 {
    let mut idx = 0;
    let mut distance = None;
    for (i, possible_key) in (&haystack).iter().enumerate() {
        let c_distance = (*possible_key as i8 - needle as i8).abs();
        if prec(possible_key, &needle) {
            match distance {
                Some(dist) if c_distance < dist => {}
                None => {}
                Some(_) => continue,
            }
            idx = i;
            distance = Some(c_distance);
        }
    }
    &haystack[idx]
}

const MAJOR_INTERVALS: [u8; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
const MINOR_INTERVALS: [u8; 8] = [0, 2, 3, 5, 7, 8, 10, 12];

impl KeySignature {
    fn get_next_semi<F: Fn(&u8, &u8) -> bool>(key: Key, ksd: u8, tones: &[u8], prec: F) -> Key {
        // Major tone gaps: 0 2 4 5 7 9 11 12
        // Check if the current key lies on one of these.
        // If it does, just go to the next the next value.
        // Otherwise, go to the closest key in that scale (up)

        let current_pos = (key.0 - ksd) % 12;
        let key_start = key.0 - current_pos;
        Key(key_start + dbg!(*get_closest(tones, current_pos, prec)))
    }

    /// Calculate one step up the scale
    fn calc_step_up(&self, key: Key, ksd: u8) -> Key {
        let interval = match self {
            Self::Major => &MAJOR_INTERVALS[1..],
            Self::Minor => &MINOR_INTERVALS[1..],
        };
        Self::get_next_semi(key, ksd, interval, |x, y| x > y)
    }

    /// Calculate one step down the scale
    fn calc_step_down(&self, key: Key, ksd: u8) -> Key {
        let interval = match self {
            Self::Major => &MAJOR_INTERVALS,
            Self::Minor => &MINOR_INTERVALS,
        };
        let next_semi = Self::get_next_semi(key, ksd, interval, |x, y| x < y);

        if next_semi.0 == key.0 {
            // Weird edge case
            Self::get_next_semi(Key(key.0 - 1), ksd, interval, |x, y| x <= y)
        } else {
            next_semi
        }
    }

    pub fn calc_down_n(&self, mut key: Key, ksd: u8, n: usize) -> Key {
        for _ in 0..n-1 {
            key = self.calc_step_down(key, ksd);
        }
        self.calc_step_down(key, ksd)
    }

    pub fn calc_up_n(&self, mut key: Key, ksd: u8, n: usize) -> Key {
        for _ in 0..n-1 {
            key = self.calc_step_up(key, ksd);
        }
        self.calc_step_up(key, ksd)
    }
}

#[test]
fn maj_semi_test() {
    let key_sig = KeySignature::Major;
    // Key signature delta 4 is C major
    assert_eq!(key_sig.calc_step_up(Key(50), 4).0, 51); // A♯4/B♭4 => B4
    assert_eq!(key_sig.calc_step_up(Key(70), 4).0, 71); // F♯6/G♭6 => G6
    assert_eq!(key_sig.calc_step_up(Key(76), 4).0, 78); // C7 => D7
    assert_eq!(key_sig.calc_step_up(Key(75), 4).0, 76); // B6 => C7
    assert_eq!(key_sig.calc_step_up(Key(40), 4).0, 42); // C4 => D4
    assert_eq!(key_sig.calc_step_up(Key(39), 4).0, 40); // B3 => C4

    assert_eq!(key_sig.calc_step_down(Key(50), 4).0, 49); // A♯4/B♭4 => A4
    assert_eq!(key_sig.calc_step_down(Key(70), 4).0, 69); // F♯6/G♭6 => F6
    assert_eq!(key_sig.calc_step_down(Key(76), 4).0, 75); // C7 => B6
    assert_eq!(key_sig.calc_step_down(Key(39), 4).0, 37); // B3 => A3
    assert_eq!(key_sig.calc_step_down(Key(42), 4).0, 40); // D4 => C4
}

#[test]
fn min_semi_test() {
    let key_sig = KeySignature::Minor;
    // Key signature delta 8 is E minor
    assert_eq!(key_sig.calc_step_up(Key(44), 8).0, 46); // E4 => F♯4/G♭4
    assert_eq!(key_sig.calc_step_up(Key(58), 8).0, 59); // G5 => E5
    assert_eq!(key_sig.calc_step_up(Key(59), 8).0, 61); // G5 => E5

    assert_eq!(key_sig.calc_step_down(Key(47), 8).0, 46); // G4 => F♯4/G♭4
}
