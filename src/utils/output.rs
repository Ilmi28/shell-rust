use std::{fs::{self, File}, io::{self, Write}, path::Path};

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    return Ok(line.trim().to_string());
}

pub fn write_stdout(message: &str, target: Option<&str>) -> io::Result<()> {
    match target {
        Some(path) => {
            let path = Path::new(path);
            fs::create_dir_all(path.parent().unwrap())?;
            let mut file = File::create(path.file_name().unwrap())?;
            file.write_all(message.as_bytes())?;
        },
        None => {
            print!("{}", message);
            io::stdout().flush()?;
        }
    }
    Ok(())
}

pub fn write_stderr(message: &str, target: Option<&str>) -> io::Result<()> {
    match target {
        Some(path) => {
            let path = Path::new(path);
            fs::create_dir_all(path.parent().unwrap())?;
            let mut file = File::create(path.file_name().unwrap())?;
            file.write_all(message.as_bytes())?;
        },
        None => {
            eprint!("{}", message);
            io::stderr().flush()?;
        }
    }
    Ok(())
}