use std::{env, process::Command, str};

fn main() {
    let args: Vec<String> = env::args().collect();

    let from_commit = &args[1];
    let to_commit = &args[2];

    let mut git = Command::new("git");

    let output = git
        .args(&["config", "--get", "remote.origin.url"])
        .output()
        .expect("process failed to execute");

    println!("status: {}", output.status);

    let org_and_repo = str::from_utf8(&output.stdout)
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(".")
        .collect::<Vec<&str>>()[0];

    let url = format!(
        "https://github.com/{}/compare/{}..{}",
        org_and_repo, from_commit, to_commit
    );

    Command::new("open")
        .arg(url)
        .spawn()
        .expect("open command failed to execute");
}
