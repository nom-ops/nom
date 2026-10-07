use std::process::Command;

pub fn exists_in_npm(cmd: &str) -> bool {
    let output = Command::new("npm").arg("help").arg(cmd).output();
  
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}
