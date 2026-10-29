#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, utils::{console, file, parser}};

fn main() {
    let command_registry = CommandRegistry::new();
    loop {
        console::write("$ ");
        let input = console::read_line();
        let (command, args) = parser::parse_command(&input);


        let result = command_registry.run(&command, &args);
        if result {
            continue;
        }

        if let Some(_) = file::get_command_path(&command) {
            file::execute_command(&command, &args);
            continue;
        }

        console::write_line(&format!("{command}: command not found"));
    }
}
