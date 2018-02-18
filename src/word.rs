
use ::std::fmt;
use ::gen::{of_kind, sequence};

use LetterKind::Consonant;
use LetterKind::Vowel;

#[derive(Clone, Copy, Debug)]
pub enum LetterKind {
    Consonant,
    Vowel,
}

impl ::rand::Rand for LetterKind {
    fn rand<R: ::rand::Rng>(r: &mut R) -> LetterKind {
        *r.choose(&[Vowel, Consonant]).unwrap()
    }
}

pub struct Word {
    pub kind: Vec<LetterKind>,
    pub data: String,
}

impl Word {
    pub fn new(kind: Vec<LetterKind>, data: String) -> Self {
        Word { kind, data }
    }

    pub fn change_len(&mut self, by: usize) {
        if by > self.data.len() {
            let mut extra = of_kind(sequence(by - self.data.len()));
            self.kind.append(&mut extra.kind);
            self.data.push_str(&extra.data);
        } else {
            self.data = self.data[0..by].to_owned();
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl ::rand::Rand for Word {
    fn rand<R: ::rand::Rng>(r: &mut R) -> Word {
        of_kind(r.choose(&::gen::default_formats()).unwrap().to_vec())
    }
}

