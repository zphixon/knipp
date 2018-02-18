
extern crate rand;

mod gen;
mod word;

pub use gen::{of_kind, default_formats};
pub use word::Word;
pub use word::LetterKind;

use rand::Rng;

pub fn random() -> Word {
    rand::random::<Word>()
}

pub fn with_sequences(num: usize, kinds: Vec<Vec<LetterKind>>) -> Vec<Word> {
    let mut rng = ::rand::thread_rng();
    let mut words = Vec::new();
    for _ in 0..num {
        words.push(gen::of_kind(rng.choose(&kinds).unwrap_or(&Vec::with_capacity(0)).to_vec()));
    }
    words
}

