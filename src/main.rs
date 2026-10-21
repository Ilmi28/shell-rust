#[allow(unused_imports)]
use std::io::{self, Write};
mod console;

fn main() {
    loop {
        console::write("$ ");
        let command = console::read_line();
        console::write_line(&format!("{command}: command not found"));
    }
}
