use std::{collections::HashMap, io};
mod exit;
mod echo;
mod pwd;
mod cd;
mod r#type;

pub struct ShellContext<'a> {
    pub is_builtin: &'a dyn Fn(&str) -> bool
}

pub trait Command {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext, stdout: Option<&str>, stderr: Option<&str>) -> io::Result<()>;
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Box<dyn Command>>
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

    pub fn run(&self, name: &str, args: &Vec<String>, stdout: Option<&str>, stderr: Option<&str>) -> io::Result<bool> {
        match self.commands.get(name) {
            Some(cmd) => {
                let ctx = ShellContext {
                    is_builtin: &|target| self.commands.contains_key(target)
                };
                cmd.execute(args, &ctx, stdout, stderr)?;
                return Ok(true);
            }
            None => {
                return Ok(false);
            }
        }
    }
}