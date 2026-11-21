use crate::{commands::{Command, ShellContext}, utils::redirect::{self}};



pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, args: &[String], ctx: &mut ShellContext) -> std::io::Result<()> {
        let mut iter = args.iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" => {
                    if let Some(completion) = iter.next() {
                        let completion_path = ctx.completions.get_path(completion);
                        if let Some(completion_path) = completion_path {
                            redirect::write_stdout(&format!("complete -C '{completion_path}' {completion}\n"), ctx.out)?;
                        } else {
                            redirect::write_stderr(&format!("complete: {completion}: no completion specification\n"), ctx.out)?
                        }
                    }
                },
                "-C" => {
                    if let Some(path) = iter.next() && let Some(completion) = iter.next() {
                        ctx.completions.register(completion, path);
                    }
                },
                "-r" => {
                    if let Some(completion) = iter.next() {
                        ctx.completions.unregister(completion);
                    }
                }
                _other => {

                }
            }
        }
        Ok(())
    }
}