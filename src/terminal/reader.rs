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

pub fn read_input(registry: &CommandRegistry) -> io::Result<String> {
    print!("{PROMPT}");
    io::stdout().flush()?;
    let mut input = String::new();
    let mut prev_key: u8 = NUL;
    loop {
        let mut chr = [0u8; 1];
        io::stdin().read(&mut chr)?;
        match chr[0] {
            NEW_LINE | ENTER => { break; }
            TAB => {
                let (command, args) = parser::parse_command(&input);

                let mut trailing_space = true;
                let mut autocomplete = if args.is_empty() {
                    autocomplete::get_command_autocomplete(&command, registry)?
                } else {
                    autocomplete::get_arg_autocomplete(&command, &args, &mut trailing_space, registry)?
                };
                autocomplete.sort();
                autocomplete.dedup();

                if args.is_empty() {
                    command_autocomplete(&autocomplete, &mut input, &prev_key, &mut chr[0])?;
                } else {
                    let prev_input = build_prev_input(&command, &args);
                    arg_autocomplete(&autocomplete, &mut input, &trailing_space, &prev_key, &mut chr[0], &prev_input)?;
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
        prev_key = chr[0];
    }
    if !input.is_empty() {
        print!("\n");
    }
    io::stdout().flush()?;
    return Ok(input);
}

fn command_autocomplete(autocomplete: &Vec<String>, input: &mut String, prev_key: &u8, curr_key: &mut u8) -> io::Result<()> {
    if autocomplete.is_empty() {
        print!("{}", BELL);
    } else if autocomplete.len() == 1 {
        clear_line();
        let suggestion = autocomplete.first().unwrap();
        *input = format!("{suggestion} ");
        print!("{PROMPT}{suggestion} ");
        *curr_key = NUL;
    } else {
        if *prev_key == TAB {
            let suggestions = autocomplete.join("  ");
            print!("\n{suggestions}");
            print!("\n{PROMPT}{input}");
            *curr_key = NUL;
            io::stdout().flush()?;
        } else {
            print!("{}", BELL);
            clear_line();
            let prefix = autocomplete::extract_common_prefix(&autocomplete)?;
            *input = format!("{prefix}");
            print!("{PROMPT}{prefix}");
        }
    }
    Ok(())
}

fn arg_autocomplete(autocomplete: &Vec<String>, input: &mut String, trailing_space: &bool, prev_key: &u8, curr_key: &mut u8, prev_input: &str) -> io::Result<()> {
    if autocomplete.is_empty() {
        print!("{}", BELL);
    } else if autocomplete.len() == 1 {
        clear_line();
        let suggestion = autocomplete.first().unwrap();
        *input = format!("{prev_input} {suggestion}");
        print!("{PROMPT}{prev_input} {suggestion}");
        if *trailing_space {
            input.push(' ');
            print!(" ");
        }
    } else {
        if *prev_key == TAB {
            let suggestions = autocomplete.join("  ");
            print!("\n{suggestions}");
            print!("\n{PROMPT}{input}");
            *curr_key = NUL;
            io::stdout().flush()?;
        } else {
            print!("{}", BELL);
            clear_line();
            let prefix = autocomplete::extract_common_prefix(&autocomplete)?;
            *input = format!("{prev_input} {prefix}");
            print!("{PROMPT}{prev_input} {prefix}");
        }
    }
    Ok(())
}


fn build_prev_input(command: &str, args: &Vec<String>) -> String {
    if args.is_empty() {
        return String::new();
    }
    let prev_args = &args[..args.len() - 1];
    if prev_args.is_empty() {
        return String::from(format!("{command}"));
    }
    else {
        let prev_args = prev_args.join(" ");
        return String::from(format!("{command} {prev_args}"));
    }
}

fn clear_line() { print!("{}", CLEAR_LINE); }