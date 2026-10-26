use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn is_executable(path: &str) -> bool {
    let metadata = fs::metadata(path).unwrap();
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    return mode & 0o111 != 0;
}