use std::io::{self};
use codecrafters_shell::{CommandRegistry, commands::ShellContext, completion::CompletionRegistry, jobs::{JobRegistry, job_helper}, terminal::{history_reg::HistoryRegistry, pipeline, reader, termios}, utils::{file, parser, writer::{self}}};

fn main() -> io::Result<()> {
    let command_registry = CommandRegistry::new();
    let mut completion_registry = CompletionRegistry::default();
    let mut job_registry = JobRegistry::default();
    let mut history = HistoryRegistry::default();
    let _terminal = termios::RawTerminal::enable_raw_mode();
    loop {
        job_helper::write_done_jobs(&mut job_registry)?;
        let input = reader::read_input(&completion_registry, &command_registry)?;

        if input.is_empty() {
            println!();
            continue;
        }

        history.add(&input);

        let (command, mut args) = parser::parse_command(&input);
        let mut output_redirect = parser::parse_output_redirect(&mut args)?;

        let mut ctx = ShellContext {
            commands: &command_registry,
            completions: &mut completion_registry,
            jobs: &mut job_registry,
            history: &history,
            out: &mut output_redirect
        };

        if args.iter().any(|arg| arg == "|") {
            let commands = pipeline::extract_commands(&command, &args);
            pipeline::run_pipeline(commands, &mut ctx)?;
            continue;
        }
        let background = parser::extract_background(&mut args);

        let result = command_registry.run(&command, &args, &mut ctx)?;
        if result {
            continue;
        }

        if file::get_command_path(&command).is_some() {
            if background {
                file::execute_command_background(&command, &args, &mut output_redirect, &mut job_registry)?;
            } else {
                file::execute_command(&command, &args, &mut output_redirect)?;
            }
            continue;
        }

        writer::write_stderr(&format!("{command}: command not found\n"), &output_redirect)?;
    }
}
