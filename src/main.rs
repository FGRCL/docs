use std::{ env, io::{BufRead, stdin}};
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let regex = Regex::new(r"([^\s^\)]+)\s*\(([^\(]+)\)\s*-\s(.*)").expect("Could not parse regex");

    let stdin: Vec<String> = stdin().lock().lines().into_iter().map(
        |line| line.unwrap()
    ).collect();

    let hay = stdin.join("\n");

    for (_, [tool, _number, description]) in regex.captures_iter(hay.as_str()).map(|c| c.extract()) {
        println!("{}{}{}", tool, args[1], description)
    }

    return Ok(());
}
