
use std::{env, io, path::Path};

use crate::{CommandRegistry, commands::OutputRedirect, consts, utils::redirect};

use super::Command;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &Vec<String>, _registry: &CommandRegistry, output_redirect: &OutputRedirect) -> io::Result<()> {
        if args.is_empty() {
            redirect::write_stderr("Error: cd requires passing an argument", output_redirect)?;
            return Ok(());
        }
        let input_path = &args[0];
        if input_path == "~" {
            let home_dir = env::var(consts::ENV_HOME).unwrap();
            handle_set_dir(&home_dir, &input_path, output_redirect)?;
            return Ok(());
        }
        else if input_path.starts_with("/") {
            handle_set_dir(&input_path, &input_path, output_redirect)?;
            return Ok(());
        }
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
        handle_set_dir(&output_path, &input_path, output_redirect)?;
        Ok(())
    }

}

fn handle_set_dir(dir: &str, input: &str, out: &OutputRedirect) -> io::Result<()> {
    if Path::new(&dir).is_dir() {
        env::set_current_dir(&dir).unwrap();
    } 
    else 
    {
        redirect::write_stderr(&format!("cd: {input}: No such file or directory\n"), out)?;
    }
    Ok(())
}