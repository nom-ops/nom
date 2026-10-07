use std::process::Command;

pub fn run(args: &[String]) -> i32 {
    let status = Command::new("npm").args(args).status();
    
    match status {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("failed to execute npm: {}", e);
            1
        }
    }
}
