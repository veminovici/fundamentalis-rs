mod sw_vers;

use std::process::Command;
use sw_vers::*;

fn is_os() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn main() {
    let os = is_os();
    println!("is os: {}", os);
}
