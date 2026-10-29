use crate::{commands::ShellContext, utils::console};
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &Vec<String>, _ctx: &ShellContext) {
        let message = args.join(" ");
        console::write_line(&message);
    }
}
