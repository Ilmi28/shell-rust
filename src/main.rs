#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, utils::{file, output, parser}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    loop {
        output::write_stdout("$ ", &None)?;
        let input = output::read_line()?;
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

        output::write_stderr(&format!("{command}: command not found\n"), &output_redirect.stderr)?;
    }
}
