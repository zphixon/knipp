
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
        .get_matches();

    let fmts = if let Some(format) = matches.value_of("format") {
        let raw_fmts: Vec<String> = format
            .trim_matches('\'')
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
        knipp::default_formats()
    };

    let num = matches.value_of("number")
        .map_or(1, |n| {
            n.parse::<usize>().unwrap_or_else(|n| {
                eprintln!("error: Number must be number: {}", n);
                std::process::exit(1);
            })
        });

    for word in knipp::with_sequences(num, fmts) {
        println!("{}", word);
    }
}

