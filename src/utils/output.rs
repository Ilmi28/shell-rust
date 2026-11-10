use std::{fs::OpenOptions, io::{self, Read, Write}};

use crate::utils::file;


const NEW_LINE:     u8 = b'\n';
const ENTER:        u8 = b'\r';
const BACKSPACE:    u8 = 127;
const TAB:          u8 = b'\t';
const NUL:          u8 = b'\0';

const CLEAR_LINE:   &str = "\x1b[2K\r";
const BELL:         &str = "\x07";

pub const PROMPT: &str = "$ ";

pub struct OutputRedirect {
    pub stdout: Option<(String, bool)>,
    pub stderr: Option<(String, bool)>
}

pub fn read_input(commands: &Vec<&str>) -> io::Result<String> {
    let mut input = String::new();
    let mut prev: u8 = NUL;
    loop {
        let mut chr = [0u8; 1];
        io::stdin().read(&mut chr)?;
        match chr[0] {
            NEW_LINE | ENTER => { break; }
            TAB => {
                let mut autocomplete = get_autocomplete(&input, commands)?;
                autocomplete.sort();
                autocomplete.dedup();
                if autocomplete.is_empty() {
                    print!("{}", BELL);
                    io::stdout().flush()?;
                    continue;
                } 
                else if autocomplete.len() == 1 {
                    clear_line();
                    let command = autocomplete.first().unwrap();
                    input = String::from(format!("{command} "));
                    print!("{PROMPT}{command} ");
                } else {
                    print!("{}", BELL);
                    handle_multiple_autocomplete(prev, &mut chr[0], autocomplete, &mut input)?;
                    io::stdout().flush()?;
                }
                io::stdout().flush()?;
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
        prev = chr[0];
    }
    if !input.is_empty() {
        print!("\n");
        io::stdout().flush()?;
    }
    return Ok(input);
}

fn handle_multiple_autocomplete(prev: u8, curr: &mut u8, autocomplete: Vec<String>, input: &mut String) -> io::Result<()> {
    if prev == TAB && *curr == TAB {
        let suggestions = autocomplete.join("  ");
        print!("\n{suggestions}");
        print!("\n{PROMPT}{input}");
        *curr = NUL;
    }
    else {
        clear_line();
        let prefix = extract_common_prefix(&autocomplete)?;
        *input = prefix;
        print!("{PROMPT}{input}");
    }
    Ok(())
}

fn extract_common_prefix(arr: &Vec<String>) -> io::Result<String> {
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

fn get_autocomplete(input: &String, commands: &Vec<&str>) -> io::Result<Vec<String>> {
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

