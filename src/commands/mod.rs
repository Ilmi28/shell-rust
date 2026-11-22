use std::{collections::HashMap, io};

use crate::{completion::CompletionRegistry, jobs::JobRegistry, utils::redirect::OutputRedirect};
mod exit;
mod echo;
mod pwd;
mod cd;
mod complete;
mod r#type;
mod jobs;

pub trait Command {
    fn execute(&self, args: &[String], ctx: &mut ShellContext) -> io::Result<()>;
}

pub struct ShellContext<'a> {
    pub commands: &'a CommandRegistry,
    pub completions: &'a mut CompletionRegistry,
    pub jobs: &'a mut JobRegistry,
    pub out: &'a OutputRedirect
}

#[derive(Default)]
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
        commands.insert("complete", Box::new(complete::CompleteCommand));
        commands.insert("jobs", Box::new(jobs::JobsCommand));

        Self { commands }
    }

    pub fn is_builtin(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    pub fn get_commands(&self) -> Vec<String> {
        self.commands
            .keys()
            .map(|&cmd| cmd.to_string())
            .collect()
    }

    pub fn run(&self, name: &str, args: &[String], ctx: &mut ShellContext) -> io::Result<bool> {
        match self.commands.get(name) {
            Some(cmd) => {
                cmd.execute(args, ctx)?;
                Ok(true)
            }
            None => {
                Ok(false)
            }
        }
    }
}