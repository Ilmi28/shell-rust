use std::io;

use crate::utils::file;


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

pub fn get_autocomplete(input: &String, commands: &Vec<&str>) -> io::Result<Vec<String>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    let mut result: Vec<String> = commands
                                        .iter()
                                        .filter(|x| x.starts_with(input))
                                        .map(|x| x.to_string()).collect();

    let mut external_autocomplete = file::get_external_commands_autocomplete(input)?;
    result.append(&mut external_autocomplete);
    Ok(result)
}