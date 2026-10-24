use std::process;
use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: &[&str]) {
        process::exit(0);
    }
}