use std::{io, process};
use crate::{CommandRegistry, commands::OutputRedirect};

use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: &[String], _registry: &CommandRegistry, _out: &OutputRedirect) -> io::Result<()> {
        process::exit(0);
    }
}