use crate::{commands::{Command, ShellContext}, utils::redirect::{self, OutputRedirect}};



pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, args: &Vec<String>, ctx: &ShellContext<'_>, out: &OutputRedirect) -> std::io::Result<()> {
        let mut iter = args.iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" => {
                    if let Some(next_arg) = iter.next() {
                        redirect::write_stderr(&format!("complete: {next_arg}: no completion specification\n"), &out.stderr)?
                    }
                },
                _other => {

                }
            }
        }
        Ok(())
    }
}