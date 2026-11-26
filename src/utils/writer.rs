use std::{fs::OpenOptions, io::{self, Write}};

pub const PROMPT: &str = "$ ";

#[derive(Default)]
pub struct OutputRedirect {
    pub stdout: Option<(String, bool)>,
    pub stderr: Option<(String, bool)>,
    pub pipein: Option<String>,
    pub pipeout: Option<String>
}

pub fn write_stdout(message: &str, target: &mut OutputRedirect) -> io::Result<()> {
    if let Some(buf) = &mut target.pipeout {
        buf.push_str(message);
        return Ok(())
    }

    match &target.stdout {
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

pub fn write_stderr(message: &str, target: &OutputRedirect) -> io::Result<()> {
    match &target.stderr {
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

