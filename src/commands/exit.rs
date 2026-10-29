use std::process;
use crate::commands::ShellContext;

use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: &Vec<String>, _ctx: &ShellContext) {
        process::exit(0);
    }
}