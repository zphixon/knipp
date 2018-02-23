use word::{LetterKind, Word};

pub static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub static CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'
];

use rand::distributions::{Range, Sample};

pub fn gen_consonant() -> u8 {
    let mut range = Range::new(0, CONSONANTS.len());
    let mut rng = ::rand::thread_rng();
    CONSONANTS[range.sample(&mut rng)] as u8
}

pub fn gen_vowel() -> u8 {
    let mut range = Range::new(0, VOWELS.len());
    let mut rng = ::rand::thread_rng();
    VOWELS[range.sample(&mut rng)] as u8
}

pub fn sequence(num: usize) -> Vec<LetterKind> {
    let mut v = Vec::new();
    for _ in 0..num {
        v.push(::rand::random::<LetterKind>());
    }
    v
}

pub fn of_kind(kind: Vec<LetterKind>) -> Word {
    let mut data = String::new();

    for l in kind.iter() {
        data.push(match *l {
            LetterKind::Consonant => gen_consonant() as char,
            LetterKind::Vowel => gen_vowel() as char,
        });
    }

    Word::new(kind, data)
}

pub fn default_formats() -> Vec<Vec<LetterKind>> {
    use LetterKind::{Consonant, Vowel};
    vec![
        vec![Consonant, Vowel, Consonant, Consonant, Vowel],
        vec![Consonant, Consonant, Vowel, Consonant, Consonant],
        vec![Consonant, Consonant, Vowel, Consonant, Vowel],
        vec![Vowel, Consonant, Vowel, Consonant, Consonant],
        vec![Vowel, Consonant, Consonant, Vowel, Vowel],
        vec![Vowel, Consonant, Consonant, Vowel, Consonant],
    ]
}
