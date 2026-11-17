use std::io;

use crate::{CommandRegistry, commands::OutputRedirect, utils::redirect};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &Vec<String>, _registry: &CommandRegistry, output_redirect: &OutputRedirect) -> io::Result<()> {
        let message = args.join(" ");
        redirect::write_stdout(&format!("{message}\n"), output_redirect)?;
        Ok(())
    }
}
