use std::io::{self};

use crate::{CommandRegistry, completion::CompletionRegistry, utils::file};


pub fn extract_common_prefix(arr: &[String]) -> io::Result<String> {
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

pub fn get_command_autocomplete(command: &str, cmd_reg: &CommandRegistry) -> io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let commands = cmd_reg.get_commands();
    let mut commands_autocomplete: Vec<String> = commands
                                            .iter()
                                            .filter(|x| x.starts_with(command))
                                            .map(|x| x.to_string()).collect();

    let mut external_autocomplete = file::get_external_commands_autocomplete(command)?;
    result.append(&mut external_autocomplete);
    result.append(&mut commands_autocomplete);
    Ok(result)
}

pub fn get_arg_autocomplete(command: &str, args: &[String], space: &mut bool, cmpl_reg: &CompletionRegistry) -> io::Result<Vec<String>> {
    let completion_path = cmpl_reg.get_path(command);
    let last_arg = args.last().unwrap();
    let mut result: Vec<String> = Vec::new();
    if let Some(path) = completion_path {
        let mut argv: [&str; 3] = [""; 3];
        argv[0] = command;
        argv[1] = last_arg;
        
        if args.len() > 1 {
            argv[2] = &args[args.len() - 2];
        } else {
            argv[2] = command;
        }
        let completions = file::execute_completer_script(&path, &argv);
        let completions: Vec<String> = completions.split('\n').map(String::from).collect();
        for completion in completions {
            if !completion.is_empty() {
                result.push(completion);
            }
        }
    } else {
        let mut files = file::get_files_autocomplete(last_arg, space)?;
        result.append(&mut files);
    }
    Ok(result)
}