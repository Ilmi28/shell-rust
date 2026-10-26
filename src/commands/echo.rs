use crate::{commands::ShellContext, console};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[&str], _ctx: &ShellContext) {
        let message = args.join(" ");
        console::write_line(&message);
    }
}
