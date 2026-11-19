use std::io::{self};

use crate::{CommandRegistry, utils::file};


pub fn extract_common_prefix(arr: &Vec<String>) -> io::Result<String> {
    if arr.is_empty() {
        return Ok(String::new());
    }

    let mut prefix = String::new();
    let first = &arr[0];

    for (n, curr_chr) in first.chars().enumerate() {
        for curr_el in arr {
            match curr_el.chars().nth(n) {
                Some(c) if c == curr_chr => {}
                _ => return Ok(prefix),
            }
        }
        prefix.push(curr_chr);
    }

    Ok(prefix)
}

pub fn get_command_autocomplete(command: &str, registry: &CommandRegistry) -> io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let commands = registry.get_commands();
    let mut commands_autocomplete: Vec<String> = commands
                                            .iter()
                                            .filter(|x| x.starts_with(command))
                                            .map(|x| x.to_string()).collect();

    let mut external_autocomplete = file::get_external_commands_autocomplete(command)?;
    result.append(&mut external_autocomplete);
    result.append(&mut commands_autocomplete);
    Ok(result)
}

pub fn get_arg_autocomplete(command: &str, args: &Vec<String>, space: &mut bool, registry: &CommandRegistry) -> io::Result<Vec<String>> {
    let completion_path = registry.get_completion_path(command);
    let last_arg = args.last().unwrap();
    let mut result: Vec<String> = Vec::new();
    if let Some(path) = completion_path {
        let mut argv: [&str; 3] = [""; 3];
        argv[0] = command;
        argv[1] = last_arg;
        if args.len() > 1 {
            argv[2] = &args[args.len() - 2];
        } 
        let completion = file::execute_completer_script(&path, &argv);
        let completion = completion.trim().to_string();
        if !completion.is_empty() {
            result.push(completion);
        }
    } else {
        let mut files = file::get_files_autocomplete(last_arg, space)?;
        result.append(&mut files);
    }
    Ok(result)
}