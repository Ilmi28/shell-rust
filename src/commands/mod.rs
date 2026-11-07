use std::{collections::HashMap, io};

use crate::utils::output::OutputRedirect;
mod exit;
mod echo;
mod pwd;
mod cd;
mod r#type;

pub struct ShellContext<'a> {
    pub is_builtin: &'a dyn Fn(&str) -> bool
}

pub trait Command {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext, out: &OutputRedirect) -> io::Result<()>;
}

pub struct CommandRegistry {
    pub commands: HashMap<&'static str, Box<dyn Command>>
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Box<dyn Command>> = HashMap::new();

        commands.insert("exit", Box::new(exit::ExitCommand));
        commands.insert("echo", Box::new(echo::EchoCommand));
        commands.insert("type", Box::new(r#type::TypeCommand));
        commands.insert("pwd",Box::new(pwd::PwdCommand));  
        commands.insert("cd", Box::new(cd::CdCommand));

        Self { commands }
    }

    pub fn run(&self, name: &str, args: &Vec<String>, out: &OutputRedirect) -> io::Result<bool> {
        match self.commands.get(name) {
            Some(cmd) => {
                let ctx = ShellContext {
                    is_builtin: &|target| self.commands.contains_key(target)
                };
                cmd.execute(args, &ctx, out)?;
                return Ok(true);
            }
            None => {
                return Ok(false);
            }
        }
    }
}