use std::{fs::OpenOptions, io::{self, Read, Write}};

use crate::utils::file;


const NEW_LINE:     u8 = b'\n';
const ENTER:        u8 = b'\r';
const BACKSPACE:    u8 = 127;
const TAB:          u8 = b'\t';

const CLEAR_LINE:   &str = "\x1b[2K\r";
const BELL:         &str = "\x07";

pub const PROMPT: &str = "$ ";

pub struct OutputRedirect {
    pub stdout: Option<(String, bool)>,
    pub stderr: Option<(String, bool)>
}

pub fn read_input(commands: &Vec<&str>) -> io::Result<String> {
    let mut input = String::new();
    loop {
        let mut chr = [0u8; 1];
        io::stdin().read(&mut chr)?;
        match chr[0] {
            NEW_LINE | ENTER => { break; }
            TAB => {
                let autocomplete = autocomplete(&input, commands)?;
                if autocomplete.is_empty() {
                    print!("{}", BELL);
                    io::stdout().flush()?;
                    continue;
                }
                print!("{PROMPT}{autocomplete} ");
                io::stdout().flush()?;
                input = String::from(format!("{autocomplete} "));
            }
            BACKSPACE => {
                clear_line();
                input.pop();
                print!("{PROMPT}{input}");
                io::stdout().flush()?;
            }
            _ => {
                input.push(chr[0] as char);
                print!("{}", chr[0] as char);
                io::stdout().flush()?
            }
        }
    }
    if !input.is_empty() {
        print!("\n");
        io::stdout().flush()?;
    }
    return Ok(input);
}

fn autocomplete(input: &String, commands: &Vec<&str>) -> io::Result<String> {
    if input.is_empty() {
        return Ok(String::new());
    }
    for command in commands {
        if command.starts_with(input) {
            clear_line();
            return Ok(command.to_string());
        }
    }
    let external_autocomplete = file::get_external_command_autocomplete(input)?;
    if let Some(autocomplete) = external_autocomplete {
        clear_line();
        return Ok(autocomplete);
    }
    Ok(String::new())
}

fn clear_line() { print!("{}", CLEAR_LINE); }

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

