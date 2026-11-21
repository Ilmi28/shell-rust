use std::collections::HashMap;

pub mod autocomplete;

#[derive(Default)]
pub struct CompletionRegistry {
    completions: HashMap<String, String>
}

impl CompletionRegistry {
    pub fn register(&mut self, command: &str, path: &str) {
        self.completions.insert(command.to_string(), path.to_string());
    }

    pub fn get_path(&self, command: &str) -> Option<String> {
        self.completions.get(command).cloned()
    }

    pub fn unregister(&mut self, command: &str) {
        self.completions.remove(command);
    }

    pub fn is_completer(&self, command: &str) -> bool {
        self.completions.contains_key(command)
    }
}