use std::process::Command;

fn main() {
    let output = Command::new("filefrag")
        .arg("-e")
        .arg("../lecture.txt")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}