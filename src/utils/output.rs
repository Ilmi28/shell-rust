use std::{fs::OpenOptions, io::{self, Write}};

use crate::{commands::OutputRedirect, utils::{file, parser}};

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    return Ok(line.trim().to_string());
}

pub fn write_stdout(message: &str, target: &Option<(String, bool)>) -> io::Result<()> {
    match target {
        Some(target) => {
            write_file(message, target)?;
        },
        None => {
            print!("{}", message);
            io::stdout().flush()?;
        }
    }
    Ok(())
}

pub fn write_stderr(message: &str, target: &Option<(String, bool)>) -> io::Result<()> {
    match target {
        Some(target) => {
            write_file(message, target)?;
        },
        None => {
            eprint!("{}", message);
            io::stderr().flush()?;
        }
    }
    Ok(())
}

pub fn get_output_redirect(args: &mut Vec<String>) -> io::Result<OutputRedirect> {
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
    Ok(output_redirect)
}

fn write_file(message: &str, target: &(String, bool)) -> io::Result<()> {
    let (path, overwrite) = target;
    
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(*overwrite)
        .append(!*overwrite)
        .open(path)?;

    file.write_all(message.as_bytes())?;    
    Ok(())
}

