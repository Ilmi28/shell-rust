#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        println!("$ ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        println!("{command}: command not found");  
        io::stdout().flush().unwrap();
    }
}
