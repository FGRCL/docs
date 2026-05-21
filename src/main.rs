use std::{ io::{ Write}, process::{ChildStdin, Command, Stdio}, };

use regex::Regex;

fn main() {
    let delimiter = " -- ";

    let apropos = Command::new("apropos")
        .arg(".")
        .output()
        .expect("failed to run apropos");

    let mut fzf_process = Command::new("fzf")
        .args([
            "--style", "full",
            "--delimiter", delimiter,
            "--accept-nth", "1",
            "--with-nth", "{1} - {2}",
            "--preview", "man --warnings=!font {1}",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run fzf");

    parse(String::from_utf8(apropos.stdout).expect("failed to read the apropos output"), fzf_process.stdin.take().expect("failed to take the fzf stdin"), delimiter);
    let fzf = fzf_process.wait_with_output().expect("failed to run fzf");
    let selection = String::from_utf8(fzf.stdout).expect("failed to read the fzf output");
    let page = selection.trim();

    let _ = Command::new("man")
        .args([page])
        .status()
        .expect("failed to run man");
}

fn parse(apropos: String, mut fzf: ChildStdin, delimiter: &str) {
    let regex = Regex::new(r"([^\s^\)]+)\s*\(([^\(]+)\)\s*-\s(.*)").expect("Could not parse regex");
    for (_, [tool, _number, description]) in regex.captures_iter(apropos.as_str()).map(|c| c.extract()) {
        // std::thread::spawn( |fzf| {
        let _ = fzf.write_all(format!("{}{}{}\n", tool, delimiter, description).as_bytes());
        // });
    }
}
