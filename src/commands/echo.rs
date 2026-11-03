use std::io;

use crate::{commands::{OutputRedirect, ShellContext}, utils::output};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &Vec<String>, _ctx: &ShellContext, output_redirect: &OutputRedirect) -> io::Result<()> {
        let message = args.join(" ");
        output::write_stdout(&format!("{message}\n"), &output_redirect.stdout)?;
        Ok(())
    }
}
