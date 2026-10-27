#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, utils::{console, file}};

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
        if result {
            continue;
        }

        if let Some(_) = file::get_command_path(command_name) {
            file::execute_command(command_name, &args);
            continue;
        }

        console::write_line(&format!("{command_name}: command not found"));
    }
}
