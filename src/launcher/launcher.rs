use std::process::Command;
use which::which;

pub fn launch(name: &str) -> Result<(), String> {
    let path = which(name).map_err(|_| format!("Command '{}' not found in PATH", name))?;

    Command::new(path)
        .spawn()
        .map_err(|e| format!("Failed to launch '{}': {}", name, e))?;

    Ok(())
}
