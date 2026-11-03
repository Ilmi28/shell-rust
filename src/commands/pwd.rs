use std::{env, io};

use crate::utils::output::{self, OutputRedirect};

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &Vec<String>, _ctx: &super::ShellContext, output_redirect: &OutputRedirect) -> io::Result<()> {
        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_str().unwrap();
        output::write_stdout(&format!("{current_dir_str}\n"), &output_redirect.stdout)?;
        Ok(())
    }
}