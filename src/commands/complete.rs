use crate::{CommandRegistry, commands::Command, utils::redirect::{self, OutputRedirect}};



pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, out: &OutputRedirect) -> std::io::Result<()> {
        let mut iter = args.iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" => {
                    if let Some(completion) = iter.next() {
                        let completion_path = registry.get_completion_path(completion);
                        if let Some(completion_path) = completion_path {
                            redirect::write_stdout(&format!("complete -C '{completion_path}' {completion}\n"), out)?;
                        } else {
                            redirect::write_stderr(&format!("complete: {completion}: no completion specification\n"), out)?
                        }
                    }
                },
                "-C" => {
                    if let Some(path) = iter.next() && let Some(completion) = iter.next() {
                        registry.set_completion(completion, path);
                    }
                },
                "-r" => {
                    if let Some(completion) = iter.next() {
                        registry.remove_completion(completion);
                    }
                }
                _other => {

                }
            }
        }
        Ok(())
    }
}