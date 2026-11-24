#[allow(unused_imports)]
use std::io::{self, Write};
use codecrafters_shell::{CommandRegistry, commands::ShellContext, completion::CompletionRegistry, jobs::{JobRegistry, job_helper}, terminal::{pipeline, reader, termios}, utils::{file, parser, redirect::{self, OutputRedirect}}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    let mut completion_registry = CompletionRegistry::default();
    let mut job_registry = JobRegistry::default();
    let _terminal = termios::RawTerminal::enable_raw_mode();
    loop {
        job_helper::write_done_jobs(&mut job_registry)?;
        let input = reader::read_input(&completion_registry, &command_registry)?;

        if input.is_empty() {
            redirect::write_stdout("\n", &OutputRedirect::default())?;
            continue;
        }
        
        let (command, mut args) = parser::parse_command(&input);
        if args.iter().any(|arg| arg == "|") {
            let commands = pipeline::extract_commands(&command, &args);
            pipeline::run_pipeline(commands)?;
            continue;
        }
        let background = parser::extract_background(&mut args);
        let output_redirect = parser::parse_output_redirect(&mut args)?;

        let mut ctx = ShellContext {
            commands: &command_registry,
            completions: &mut completion_registry,
            jobs: &mut job_registry,
            out: &output_redirect
        };

        let result = command_registry.run(&command, &args, &mut ctx)?;
        if result {
            continue;
        }

        if file::get_command_path(&command).is_some() {
            if background {
                file::execute_command_background(&command, &args, &output_redirect,&mut job_registry)?;
            } else {
                file::execute_command(&command, &args, &output_redirect)?;
            }
            continue;
        }

        redirect::write_stderr(&format!("{command}: command not found\n"), &output_redirect)?;
    }
}
