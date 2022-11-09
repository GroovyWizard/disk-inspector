use std::process::Command;
use std::str;

pub fn get_disk_usage() -> String {
    let home_directory: String = match dirs::home_dir() {
        Some(path) => path.to_string_lossy().to_string(),
        None => panic!("Error"),
    };

    let output = Command::new("du")
        .arg("-sh")
        .arg(home_directory)
        .output()
        .expect("Failed to execute command!");
    let parsed_output = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Error"),
    };

    return parsed_output.to_owned();
}
