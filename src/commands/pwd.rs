use std::{env, io};

use crate::{commands::ShellContext, utils::writer::{self}};

use super::Command;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) -> io::Result<()> {
        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_str().unwrap();
        writer::write_stdout(&format!("{current_dir_str}\n"), ctx.out)?;
        Ok(())
    }
}