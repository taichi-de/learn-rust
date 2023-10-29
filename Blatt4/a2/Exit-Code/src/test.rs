use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let mut child = Command::new("./programm")
        .arg("-e")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start the program.");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin.");
        stdin.write_all(b"Hello, external program!\n").expect("Failed to write to stdin.");
        stdin.write_all(b".\n").expect("Failed to write to stdin.");
    }

    let output = child.wait_with_output().expect("Failed to read stdout.");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
