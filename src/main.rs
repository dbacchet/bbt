
use std::process::Command;
// use regex::Regex;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn main() {
    let output = Command::new("git").arg("log").arg("--oneline").output().unwrap();

    if !output.status.success() {
        println!("Command executed with failing error code");
        return;
    }

    println!("{}", String::from_utf8(output.stdout).unwrap());

    // let pattern = Regex::new(r"(?x)
    //                            ([0-9a-fA-F]+) # commit hash
    //                            (.*)           # The commit message")?;
    //
    // String::from_utf8(output.stdout)?
    //     .lines()
    //     .filter_map(|line| pattern.captures(line))
    //     .map(|cap| {
    //              Commit {
    //                  hash: cap[1].to_string(),
    //                  message: cap[2].trim().to_string(),
    //              }
    //          })
    //     .take(5)
    //     .for_each(|x| println!("{:?}", x));

}
