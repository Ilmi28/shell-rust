use std::io::{self, Stdin, Stdout, Write};

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

pub fn write(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

pub fn write_line(message: &str) {
    println!("{}", message);
    io::stdout().flush().unwrap();
}