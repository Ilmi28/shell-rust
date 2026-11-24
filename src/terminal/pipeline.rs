use std::{io::{self}, process::{ChildStdout, Command, Stdio}};


#[derive(Default)]
pub struct PipelineCommand {
    pub command: String,
    pub args: Vec<String>,
}

pub fn run_pipeline(commands: Vec<PipelineCommand>) -> io::Result<()> {
    let mut prev_out: Option<ChildStdout> = None;
    let mut children = Vec::new();
    for (i, command) in commands.iter().enumerate() {
        let mut cmd = Command::new(command.command.clone());
        cmd.args(command.args.clone());
        if let Some(out) = prev_out.take() {
            cmd.stdin(out);
        }

        let is_last = i == commands.len() - 1;

        if !is_last {
            cmd.stdout(Stdio::piped());
        }
        let mut child = cmd.spawn()?;
        
        if !is_last {
            prev_out = child.stdout.take();
        }

        children.push(child);
    }

    for mut child in children {
        child.wait()?;
    }
    Ok(())
}

pub fn extract_commands(command: &str, args: &[String]) -> Vec<PipelineCommand> {
    let mut result: Vec<PipelineCommand> = Vec::new();
    for (i, args) in args.split(|arg| arg == "|").enumerate() {
        let mut cmd = if i == 0 {
            PipelineCommand { command: command.to_string().clone(), args: Vec::new() }
        } else {
            PipelineCommand::default()
        };
        for (j, arg) in args.iter().enumerate() {
            if i == 0 {
                cmd.args.push(arg.clone())
            } else if i > 0 && j == 0 {
                cmd.command = arg.clone()
            } else {
                cmd.args.push(arg.clone())
            }
        }
        result.push(cmd);
    }
    result
}