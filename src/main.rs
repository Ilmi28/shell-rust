#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, utils::{output, file, parser}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    loop {
        output::write_stdout("$ ", None)?;
        let input = output::read_line()?;
        let (command, mut args) = parser::parse_command(&input);
        let stdout = parser::extract_stdout(&mut args);
        if let Some(path) = &stdout {
            file::create_file(path)?;
        } 
        let stderr = parser::extract_stderr(&mut args);
        if let Some(path) = &stderr {
            file::create_file(path)?;
        }
        let result = command_registry.run(&command, &args, stdout.as_deref(), stderr.as_deref())?;
        if result {
            continue;
        }

        if let Some(_) = file::get_command_path(&command) {
            file::execute_command(&command, &args, stdout.as_deref(), stderr.as_deref())?;
            continue;
        }

        output::write_stderr(&format!("{command}: command not found\n"), stderr.as_deref())?;
    }
}
