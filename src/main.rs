extern crate clap;
extern crate knipp;

use clap::{App, Arg};

fn main() {
    let matches = App::new("knipp")
        .version("0.1")
        .about("Generate some words.")
        .author("Zack H <zackh@firemail.cc>")
        .arg(Arg::with_name("format")
             .short("f")
             .long("format")
             .help("Specify formats separated by semicolon")
             .long_help("You can specify the format of a random word by using a combination of 'c' and\n\
                         'v' to mean consonants and vowels. These are separated by semicolons.\n\
                         This option is incompatible with --random and --length.")
             .conflicts_with_all(&["random", "length"])
             .takes_value(true))
        .arg(Arg::with_name("number")
             .short("n")
             .long("number")
             .help("Number of words to generate")
             .takes_value(true))
        .arg(Arg::with_name("random")
             .short("r")
             .long("random")
             .help("Use a completely random sequence"))
        .arg(Arg::with_name("length")
             .short("l")
             .long("length")
             .help("Generate with a specific length")
             .long_help("Generate a word with the given length.\n\
                         Used with --random, the pattern will change for each word.\n\
                         Used without random, the pattern will be the same.")
             .takes_value(true))
        .get_matches();

    let len = matches.value_of("length").map(|n| {
        n.parse::<usize>().unwrap_or_else(|n| {
            eprintln!("error: Length must be number: {}", n);
            std::process::exit(1);
        })
    });

    let fmts = if let Some(format) = matches.value_of("format") {
        let raw_fmts: Vec<String> = format
            .trim_matches('\'')
            .trim_matches('\"')
            .split(";")
            .filter(|&s| s != "")
            .map(|s| s.to_lowercase())
            .collect();

        let mut fmts = Vec::new();

        for raw_fmt in raw_fmts {
            let mut fmt = Vec::new();
            for c in raw_fmt.chars() {
                fmt.push(match c {
                    'c' => knipp::LetterKind::Consonant,
                    'v' => knipp::LetterKind::Vowel,
                    _ => {
                        eprintln!("error: Can only use v and c in format specifier");
                        std::process::exit(1);
                    }
                });
            }
            fmts.push(fmt);
        }

        fmts
    } else {
        if let Some(len) = len {
            vec![knipp::gen::sequence(len)]
        } else {
            knipp::default_formats()
        }
    };

    let num = matches.value_of("number").map_or(1, |n| {
        n.parse::<usize>().unwrap_or_else(|n| {
            eprintln!("error: Number must be number: {}", n);
            std::process::exit(1);
        })
    });

    if matches.is_present("random") {
        for mut word in knipp::random_sequences(num) {
            if let Some(len) = len {
                word.change_len(len);
            }
            println!("{}", word);
        }
    } else {
        for mut word in knipp::with_sequences(num, fmts) {
            if let Some(len) = len {
                word.change_len(len);
            }
            println!("{}", word);
        }
    }
}
