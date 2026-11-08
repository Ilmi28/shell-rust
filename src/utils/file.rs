use std::fs::File;
use std::{env, path::Path};
use std::process::Command;
use std::{fs, io};
use std::os::unix::fs::PermissionsExt;

use crate::utils::output::OutputRedirect;
use crate::consts;
use crate::utils::{output, file};

pub fn is_executable(path: &str) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(data) => data,
        Err(_) => return false
    };
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    return metadata.is_file() && mode & 0o111 != 0;
}

pub fn get_command_path(command_name: &str) -> Option<String> {
    let path_env = env::var(consts::ENV_PATH).unwrap();
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

pub fn get_external_commands_autocomplete(input: &str) -> io::Result<Vec<String>> {
    let mut commands: Vec<String> = Vec::new();
    let path_env = env::var(consts::ENV_PATH).unwrap();
    let paths: Vec<&str> = path_env.split([':', ';']).collect();
    for path in paths {
        let command_paths = fs::read_dir(path)?;
        for command_path_result in command_paths {
            let command_path = match command_path_result {
                Ok(entry) => entry,
                Err(_) => continue
            };
            let path = command_path.path();
            let path_str = path.to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with(input) && is_executable(&path_str) {
                commands.push(file_name.to_string());
            }
        }
    }
    Ok(commands)
}

pub fn execute_command(command: &str, args: &Vec<String>, output_redirect: &OutputRedirect) -> io::Result<()> {
    let mut ext_command = Command::new(command);
    let result = ext_command.args(args).output();
    match result {
        Ok(output) => {
            if !output.stdout.is_empty() {
                output::write_stdout(&String::from_utf8_lossy(&output.stdout), &output_redirect.stdout)?
            }

            if !output.stderr.is_empty() {
                output::write_stderr(&String::from_utf8_lossy(&output.stderr), &output_redirect.stderr)?
            }
        }
        Err(_) => {
            output::write_stderr(&format!("{}: command not found\n", command), &None)?;
        }
    }
    Ok(())
}

pub fn create_file_if_not_exists(path: &str) -> io::Result<File> {
    let path = Path::new(path);
    if Path::exists(path) {
        let file = File::open(path)?;
        return Ok(file)
    }
    fs::create_dir_all(path.parent().unwrap())?;
    let file = File::create(path)?;
    return Ok(file)
}