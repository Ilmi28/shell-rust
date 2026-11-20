use std::{cell::RefCell, collections::HashMap, io};

use crate::utils::redirect::OutputRedirect;
mod exit;
mod echo;
mod pwd;
mod cd;
mod complete;
mod r#type;
mod jobs;

pub trait Command {
    fn execute(&self, args: &Vec<String>, registry: &CommandRegistry, out: &OutputRedirect) -> io::Result<()>;
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Box<dyn Command>>,
    completions: RefCell<HashMap<String, String>>
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Box<dyn Command>> = HashMap::new();
        let completions: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());

        commands.insert("exit", Box::new(exit::ExitCommand));
        commands.insert("echo", Box::new(echo::EchoCommand));
        commands.insert("type", Box::new(r#type::TypeCommand));
        commands.insert("pwd",Box::new(pwd::PwdCommand));  
        commands.insert("cd", Box::new(cd::CdCommand));
        commands.insert("complete", Box::new(complete::CompleteCommand));
        commands.insert("jobs", Box::new(jobs::JobsCommand));

        Self { commands, completions }
    }

    pub fn set_completion(&self, command: &str, path: &str) {
        self.completions.borrow_mut().insert(command.to_string(), path.to_string());
    }

    pub fn get_completion_path(&self, command: &str) -> Option<String> {
        self.completions.borrow().get(command).cloned()
    }

    pub fn remove_completion(&self, command: &str) {
        self.completions.borrow_mut().remove(command);
    }

    pub fn is_completer(&self, command: &str) -> bool {
        self.completions.borrow().contains_key(command)
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

    pub fn run(&self, name: &str, args: &Vec<String>, out: &OutputRedirect) -> io::Result<bool> {
        match self.commands.get(name) {
            Some(cmd) => {
                cmd.execute(args, self, out)?;
                return Ok(true);
            }
            None => {
                return Ok(false);
            }
        }
    }
}