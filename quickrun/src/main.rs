use std::{
    io::Stdin,
    process::{Command, Stdio},
};

fn main() {
    Command::new("buildkitd")
        .args(&[""])
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
}
