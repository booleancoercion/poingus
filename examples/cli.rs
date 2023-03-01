use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

    let dictionary = get_dictionary(&dictionary)?;
    let mut output: Vec<&str> =
        unnamed_discord_thing::get_matches(&username, nickname.as_deref(), &dictionary);
    output.sort_unstable_by_key(|word| std::cmp::Reverse(word.len()));

    println!("The following words match the input: {}", output.join(", "));

    Ok(())
}

fn get_dictionary(filename: &str) -> Result<Vec<String>, std::io::Error> {
    BufReader::new(File::open(filename)?)
        .lines()
        .filter(|line| line.as_deref().map_or(true, |line| line.len() > 2))
        .collect()
}
