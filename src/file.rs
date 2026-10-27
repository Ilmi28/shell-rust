use std::path::Path;
use std::process::Command;
use std::{env, fs};
use std::os::unix::fs::PermissionsExt;

use crate::file;

pub fn is_executable(path: &str) -> bool {
    let metadata = fs::metadata(path).unwrap();
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    return metadata.is_file() && mode & 0o111 != 0;
}

pub fn get_command_path(command_name: &str) -> Option<String> {
    let path_env = env::var("PATH").unwrap();
    let paths: Vec<&str> = path_env.split([':', ';']).collect();
    for path in paths {
        let full_path = &format!("{path}/{command_name}");
        let path = Path::new(full_path);
        if path.exists() && file::is_executable(full_path) {
            return Some(full_path.to_string());
        }
    }
    None
}

pub fn execute_command(command_path: &str, args: &[&str]) {
    let _ = Command::new(command_path)
        .args(args)
        .output();
}