use std::process::Command;

pub fn init(path: &str) {
    std::fs::write(format!("{}.gitignore", path), "target/\nCargo.lock")
        .expect("Failed to write \".gitignore\"");

    Command::new("git").args(["init"]).output().unwrap();
}
