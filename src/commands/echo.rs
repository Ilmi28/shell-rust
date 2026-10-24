use crate::console;
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[&str]) {
        let message = args.join(" ");
        console::write_line(&message);
    }
}
