use std::io::{self, Read, Write};

use crate::{CommandRegistry, terminal::autocomplete, utils::parser};

const NEW_LINE:     u8 = b'\n';
const ENTER:        u8 = b'\r';
const BACKSPACE:    u8 = 127;
const TAB:          u8 = b'\t';
const NUL:          u8 = b'\0';

const CLEAR_LINE:   &str = "\x1b[2K\r";
const BELL:         &str = "\x07";

pub const PROMPT: &str = "$ ";

pub fn read_input(commands: &CommandRegistry) -> io::Result<String> {
    print!("{PROMPT}");
    io::stdout().flush()?;
    let mut input = String::new();
    let mut prev: u8 = NUL;
    loop {
        let mut chr = [0u8; 1];
        io::stdin().read(&mut chr)?;
        match chr[0] {
            NEW_LINE | ENTER => { break; }
            TAB => {
                let (command, args) = parser::parse_command(&input);
                let mut autocomplete = autocomplete::get_autocomplete(&command, &args, commands)?;
                autocomplete.sort();
                autocomplete.dedup();
                if autocomplete.is_empty() {
                    print!("{}", BELL);
                    io::stdout().flush()?;
                    continue;
                } 
                else if autocomplete.len() == 1 {
                    handle_single_autocomplete(&command, &args, &mut input, autocomplete);
                } else {
                    handle_multiple_autocomplete(prev, &mut chr[0], autocomplete, &mut input)?;
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
    }
    io::stdout().flush()?;
    return Ok(input);
}

fn handle_single_autocomplete(command: &String, args: &Vec<String>, input: &mut String, autocomplete: Vec<String>) {
    clear_line();
    let suggestion = autocomplete.first().unwrap();

    if args.is_empty() {
        *input = String::from(format!("{suggestion} "));
        print!("{PROMPT}{suggestion} ");
    }
    else {
        let prev_args = &args[..args.len() - 1];
        if prev_args.is_empty() {
            *input = String::from(format!("{command} {suggestion} "));
        }
        else {
            let prev_args = prev_args.join(" ");
            *input = String::from(format!("{command} {prev_args} {suggestion} "));
        }
        print!("{PROMPT}{input}")
    }
}

fn handle_multiple_autocomplete(prev: u8, curr: &mut u8, autocomplete: Vec<String>, input: &mut String) -> io::Result<()> {
    print!("{}", BELL);
    if prev == TAB && *curr == TAB {
        let suggestions = autocomplete.join("  ");
        print!("\n{suggestions}");
        print!("\n{PROMPT}{input}");
        *curr = NUL;
    }
    else {
        clear_line();
        let prefix = autocomplete::extract_common_prefix(&autocomplete)?;
        *input = prefix;
        print!("{PROMPT}{input}");
    }
    Ok(())
}

fn clear_line() { print!("{}", CLEAR_LINE); }