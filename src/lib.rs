extern crate rand;

pub mod gen;
pub mod word;

pub use gen::{default_formats, of_kind};
pub use word::Word;
pub use word::LetterKind;

use rand::Rng;
use rand::distributions::{Range, Sample};

pub fn random() -> Word {
    rand::random::<Word>()
}

pub fn with_sequences(num: usize, kinds: Vec<Vec<LetterKind>>) -> Vec<Word> {
    let mut rng = rand::thread_rng();
    let mut words = Vec::new();
    for _ in 0..num {
        words.push(gen::of_kind(
            rng.choose(&kinds)
                .unwrap_or(&Vec::with_capacity(0))
                .to_vec(),
        ));
    }
    words
}

pub fn random_sequences(num: usize) -> Vec<Word> {
    let mut range = Range::new(3, 7);
    let mut rng = rand::thread_rng();
    let mut words = Vec::new();
    for _ in 0..num {
        words.push(gen::of_kind(gen::sequence(range.sample(&mut rng))));
    }
    words
}
