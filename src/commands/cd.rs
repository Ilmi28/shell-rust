
use std::{env, path::Path};

use crate::utils::console;

use super::Command;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[&str], _ctx: &super::ShellContext) {
        let input_path = args[0];
        let path_parts: Vec<&str> = input_path.split('/').collect();
        let curr_path = env::current_dir().unwrap();
        let mut output_path_parts: Vec<&str> = curr_path.to_str().unwrap().split('/').collect();

        for part in path_parts {
            match part {
                "." | "" => {}
                ".." => {
                    output_path_parts.pop();
                }
                _ => output_path_parts.push(part)
            }
        }
        let output_path = output_path_parts.join("/");
        if Path::new(&output_path).is_dir() {
            env::set_current_dir(&output_path).unwrap();
        } 
        else {
            console::write_line(&format!("cd: {input_path}: No such file or directory"));
        }
    }
}