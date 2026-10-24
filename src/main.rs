#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{console, CommandRegistry};

fn main() {
    let command_registry = CommandRegistry::new();
    loop {
        console::write("$ ");
        let input = console::read_line();
        let mut parts = input.trim().split_whitespace();
        let command_name = match parts.next() {
            Some(name) => name,
            None => continue
        };
        let args: Vec<&str> = parts.collect();

        let result = command_registry.run(&command_name, &args);
        if !result {
            console::write_line(&format!("{command_name}: command not found"));
        }
    }
}
