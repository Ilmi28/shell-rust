use std::{env, io};

use crate::utils::output;

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &Vec<String>, _ctx: &super::ShellContext, stdout: Option<&str>) -> io::Result<()> {
        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_str().unwrap();
        output::write_stdout(&format!("{current_dir_str}\n"), stdout)?;
        Ok(())
    }
}