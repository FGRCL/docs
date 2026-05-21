use std::{ process::{ Command, Stdio} };


fn main() {
    let tldr_list = Command::new("tldr")
        .arg("--list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run tldr --list");

    let fzf = Command::new("fzf")
        .args([
            "--style", "full",
            "--preview", "tldr {1}",
        ])
        .stdin(tldr_list.stdout.expect(""))
        .output()
        .expect("failed to run fzf");

    let selection = String::from_utf8(fzf.stdout).expect("failed to read the fzf output");
    let page = selection.trim();

    let _ = Command::new("tldr")
        .args([
            page,
            "--pager"
        ])
        .status()
        .expect("failed to run man");
}
