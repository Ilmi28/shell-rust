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

pub fn get_autocomplete(command: &String, args: &Vec<String>, registry: &CommandRegistry) -> io::Result<Vec<String>> {
    if command.is_empty() {
        return Ok(Vec::new());
    }

    if args.is_empty() {
        let commands = registry.get_commands();
        let mut result: Vec<String> = commands
                                            .iter()
                                            .filter(|x| x.starts_with(command))
                                            .map(|x| x.to_string()).collect();

        let mut external_autocomplete = file::get_external_commands_autocomplete(command)?;
        result.append(&mut external_autocomplete);
        return Ok(result);
    }
    else {
        let curr_arg = args.last().unwrap();
        let files = file::get_filenames_in_curr_folder()?;
        for file in files {
            if file.starts_with(curr_arg) {
                return Ok(vec![file.clone()])
            }
        }
    }
    Ok(Vec::new())
}