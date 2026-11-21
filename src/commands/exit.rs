use std::{io, process};
use crate::commands::ShellContext;

use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: &[String], _ctx: &mut ShellContext) -> io::Result<()> {
        process::exit(0);
    }
}