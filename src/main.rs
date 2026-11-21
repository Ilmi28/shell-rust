#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, commands::ShellContext, completion::CompletionRegistry, terminal::{reader, termios}, utils::{file, parser, redirect::{self, OutputRedirect}}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    let mut completion_registry = CompletionRegistry::default();
    let _terminal = termios::RawTerminal::enable_raw_mode();
    loop {
        let input = reader::read_input(&completion_registry, &command_registry)?;

        if input.is_empty() {
            redirect::write_stdout("\n", &OutputRedirect::default())?;
            continue;
        }
        
        let (command, mut args) = parser::parse_command(&input);
        let background = parser::extract_background(&mut args);
        let output_redirect = parser::parse_output_redirect(&mut args)?;

        let mut ctx = ShellContext {
            commands: &command_registry,
            completions: &mut completion_registry,
            out: &output_redirect
        };

        let result = command_registry.run(&command, &args, &mut ctx)?;
        if result {
            continue;
        }

        if file::get_command_path(&command).is_some() {
            file::execute_command(&command, &args, &output_redirect, background)?;
            continue;
        }

        redirect::write_stderr(&format!("{command}: command not found\n"), &output_redirect)?;
    }
}
