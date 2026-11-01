use std::{env, io};

use crate::utils::output;

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &Vec<String>, _ctx: &super::ShellContext, stdout: Option<&str>) -> io::Result<()> {
        if let Ok(value) = env::current_dir() {
            output::write_stdout(&value.to_str().unwrap(), stdout)?;
        }
        Ok(())
    }
}