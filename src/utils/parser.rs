use std::io;

use crate::utils::{file, redirect::OutputRedirect, parser};


const DOUBLE_QUOTE_ESCAPE_CHARS: [u8; 2] = [b'"', b'\\'];

pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut parsed_parts: Vec<String> = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut current_arg: Vec<u8> = Vec::new();
    let mut escape_indexes: Vec<usize> = Vec::new();
    let input = input.as_bytes();
    for i in 0..input.len() {
        let chr = input[i];
        let is_backslash = i != 0 && input[i-1] == b'\\' && !single_quote && !escape_indexes.contains(&(i-1));
        if is_backslash {
            if !double_quote || DOUBLE_QUOTE_ESCAPE_CHARS.contains(&chr) {
                escape_indexes.push(i);
                current_arg.push(chr);
            }
            continue;
        }

        if chr == b'\\' && !single_quote { continue }

        if chr == b'"' && !single_quote {
            double_quote = !double_quote;
            continue;
        }
        
        if chr == b'\'' && !double_quote {
            single_quote = !single_quote;
            continue;
        }

        if chr == b' ' && !(single_quote || double_quote) {
            if !current_arg.is_empty() {
                let bytes = std::mem::take(&mut current_arg);
                parsed_parts.push(String::from_utf8(bytes).unwrap());
            }
            continue;
        }
        current_arg.push(chr);
    }
    parsed_parts.push(String::from_utf8(current_arg).unwrap());
    if parsed_parts.is_empty() {
        return (String::new(), Vec::new());
    }
    let command = parsed_parts.remove(0);
    return (command, parsed_parts);
}

fn extract_stdout(args: &mut Vec<String>) -> Option<String> {
    if args.is_empty() {
        return None;
    }

    for i in 0..args.len()-1 {
        if args[i] == ">" || args[i] == "1>" {
            args.remove(i);
            let target = args.remove(i);
            return Some(target);
        }
    }
    None
}

fn extract_stderr(args: &mut Vec<String>) -> Option<String> {
    if args.is_empty() {
        return None;
    }

    for i in 0..args.len()-1 {
        if args[i] == "2>" {
            args.remove(i);
            let target = args.remove(i);
            return Some(target);
        }
    }
    None
}

fn extract_stdout_append(args: &mut Vec<String>) -> Option<String> {
    if args.is_empty() {
        return None;
    }

    for i in 0..args.len()-1 {
        if args[i] == ">>" || args[i] == "1>>" {
            args.remove(i);
            let target = args.remove(i);
            return Some(target);
        }
    }
    None
}

fn extract_stderr_append(args: &mut Vec<String>) -> Option<String> {
    if args.is_empty() {
        return None;
    }

    for i in 0..args.len()-1 {
        if args[i] == "2>>" {
            args.remove(i);
            let target = args.remove(i);
            return Some(target);
        }
    }
    None
}

pub fn parse_output_redirect(args: &mut Vec<String>) -> io::Result<OutputRedirect> {
    let mut output_redirect = OutputRedirect {
        stdout: None,
        stderr: None
    };

    let stdout = parser::extract_stdout(args);
    if let Some(path) = &stdout {
        file::create_file_if_not_exists(path)?;
        output_redirect.stdout = Some((path.to_string(), true))
    } 
    let stderr = parser::extract_stderr(args);
    if let Some(path) = &stderr {
        file::create_file_if_not_exists(path)?;
        output_redirect.stderr = Some((path.to_string(), true))
    }
    let stdout_append = parser::extract_stdout_append(args);
    if let Some(path) = &stdout_append {
        file::create_file_if_not_exists(path)?;
        output_redirect.stdout = Some((path.to_string(), false))
    }
    let stderr_append = parser::extract_stderr_append(args);
    if let Some(path) = &stderr_append {
        file::create_file_if_not_exists(path)?;
        output_redirect.stderr = Some((path.to_string(), false))
    }
    Ok(output_redirect)
}