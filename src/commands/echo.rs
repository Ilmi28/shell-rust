use std::io;

use crate::{commands::ShellContext, utils::output};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &Vec<String>, _ctx: &ShellContext, stdout: Option<&str>) -> io::Result<()> {
        let message = args.join(" ");
        output::write_stdout(&format!("{message}\n"), stdout)?;
        Ok(())
    }
}
