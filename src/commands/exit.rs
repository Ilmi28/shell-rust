use std::{io, process};
use crate::commands::{OutputRedirect, ShellContext};

use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _args: &Vec<String>, _ctx: &ShellContext, _out: &OutputRedirect) -> io::Result<()> {
        process::exit(0);
    }
}