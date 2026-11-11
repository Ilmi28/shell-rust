#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, terminal::{reader, termios}, utils::{file, parser, redirect::{self}}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    let _terminal = termios::RawTerminal::enable_raw_mode();
    loop {
        let input = reader::read_input(&command_registry)?;
        if input.is_empty() {
            redirect::write_stdout("\n", &None)?;
            continue;
        }

        let (command, mut args) = parser::parse_command(&input);

        let output_redirect = parser::parse_output_redirect(&mut args)?;

        let result = command_registry.run(&command, &args, &output_redirect)?;
        if result {
            continue;
        }

        if let Some(_) = file::get_command_path(&command) {
            file::execute_command(&command, &args, &output_redirect)?;
            continue;
        }

        redirect::write_stderr(&format!("{command}: command not found\n"), &output_redirect.stderr)?;
    }
}
