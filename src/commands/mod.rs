use std::collections::HashMap;
mod exit;
mod echo;

pub trait Command {
    fn execute(&self, args: &[&str]);
}

pub struct CommandRegistry {
    commands: HashMap<&'static str, Box<dyn Command>>
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut commands: HashMap<&'static str, Box<dyn Command>> = HashMap::new();

        commands.insert("exit", Box::new(exit::ExitCommand));
        commands.insert("echo", Box::new(echo::EchoCommand));  

        Self { commands }
    }

    pub fn run(&self, name: &str, args: &[&str]) -> bool {
        match self.commands.get(name) {
            Some(cmd) => {
                cmd.execute(args);
                return true;
            }
            None => {
                return false;
            }
        }
    }
}