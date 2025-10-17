use std::process::Command;

// Command injection vulnerability
pub fn execute_command(cmd: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(cmd).output().expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}
