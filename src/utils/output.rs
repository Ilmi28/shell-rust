use std::{fs::OpenOptions, io::{self, Read, Write}};

const NEW_LINE:  u8 = b'\n';
const ENTER:     u8 = b'\r';
const BACKSPACE: u8 = 127;
const TAB:       u8 = b'\t';


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
                clear_line();
                let autocomplete = autocomplete(&input, commands);
                print!("$ {autocomplete} ");
                io::stdout().flush()?;
                input = autocomplete;
            }
            BACKSPACE => {
                clear_line();
                input.pop();
                print!("$ {input}");
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

fn autocomplete(input: &String, commands: &Vec<&str>) -> String {
    for command in commands {
        if command.starts_with(input) {
            return command.to_string();
        }
    }
    String::new()
}

fn clear_line() { print!("\x1b[2K\r"); }

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

