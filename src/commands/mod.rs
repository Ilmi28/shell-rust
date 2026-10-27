use std::collections::HashMap;
mod exit;
mod echo;
mod pwd;
mod cd;
mod r#type;

pub struct ShellContext<'a> {
    pub is_builtin: &'a dyn Fn(&str) -> bool
}

pub trait Command {
    fn execute(&self, args: &[&str], ctx: &ShellContext);
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

    pub fn run(&self, name: &str, args: &[&str]) -> bool {
        match self.commands.get(name) {
            Some(cmd) => {
                let ctx = ShellContext {
                    is_builtin: &|target| self.commands.contains_key(target)
                };
                cmd.execute(args, &ctx);
                return true;
            }
            None => {
                return false;
            }
        }
    }
}