use std::env;

use crate::{console, consts};

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[&str], _ctx: &super::ShellContext) {
        if let Ok(value) = env::var(consts::ENV_CURR_PATH) {
            console::write_line(&value);
        }
    }
}