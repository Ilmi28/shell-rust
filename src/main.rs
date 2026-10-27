use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, console, consts, file};

fn main() {
    if let Ok(current_dir) = env::current_dir() {
        unsafe {
            env::set_var(consts::ENV_CURR_PATH, &current_dir)
        }
    }

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

        if let Some(_) = file::get_command_path(command_name) {
            file::execute_command(command_name, &args);
            continue;
        }


        let result = command_registry.run(&command_name, &args);
        if !result {
            console::write_line(&format!("{command_name}: command not found"));
        }
    }
}
