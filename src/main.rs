use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = rustyline::DefaultEditor::new()?;

    let Some(dictionary) = std::env::args().nth(1) else {
        eprintln!("Please pass a dictionary file.");
        return Ok(());
    };

    let username = loop {
        let Ok(username) = rl.readline("Enter a username: ") else {
            eprintln!("You must enter a username.");
            continue
        };

        break username;
    };

    let nickname = match rl.readline("Enter an optional nickname (leave empty for none): ") {
        Ok(nickname) if nickname.trim().is_empty() => None,
        Err(_) => None,
        Ok(nickname) => Some(nickname),
    };

    let username = canonicalize_name(&username);
    let nickname = nickname.as_deref().map(canonicalize_name);

    let dictionary = get_dictionary(&dictionary)?;
    let mut output: Vec<&str> = run(&username, nickname.as_deref(), &dictionary);
    output.sort_unstable_by_key(|word| std::cmp::Reverse(word.len()));

    println!("The following words match the input: {}", output.join(", "));

    Ok(())
}

fn canonicalize_name(name: &str) -> Vec<u8> {
    name.chars()
        .filter_map(|ch| {
            ch.is_ascii_alphabetic()
                .then(|| ch.to_ascii_lowercase() as u8)
        })
        .collect()
}

fn get_dictionary(filename: &str) -> Result<Vec<String>, std::io::Error> {
    BufReader::new(File::open(filename)?)
        .lines()
        .filter(|line| line.as_deref().map_or(true, |line| line.len() > 2))
        .collect()
}

fn run<'a, S: AsRef<str> + Sync>(
    username: &[u8],
    nickname: Option<&[u8]>,
    dictionary: &'a [S],
) -> Vec<&'a str> {
    dictionary
        .into_par_iter()
        .map(|s| s.as_ref())
        .filter(|word| {
            is_subsequence(word.as_bytes(), username)
                || nickname.map_or(false, |nickname| is_subsequence(word.as_bytes(), nickname))
        })
        .collect()
}

// courtesy of orlp
pub fn is_subsequence(needle: &[u8], haystack: &[u8]) -> bool {
    needle
        .iter()
        .scan(0, |pos, ch| {
            *pos += 1 + haystack.iter().skip(*pos).position(|x| x == ch)?;
            Some(true)
        })
        .count()
        == needle.len()
}
