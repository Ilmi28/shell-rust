use std::{io::{self, Read, Write}, process::{Child, ChildStdout, Command as ProcessCommand, Stdio}};

use crate::{commands::{Command, ShellContext}, utils::{file, writer::{self}}};


#[derive(Default)]
pub struct PipelineCommand {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Default)]
struct PrevOut {
    pub child: Option<ChildStdout>,
    pub builtin: Option<String>
}

pub fn run_pipeline(commands: Vec<PipelineCommand>, ctx: &mut ShellContext) -> io::Result<()> {
    let mut prev_out = PrevOut::default();
    let mut children = Vec::new();
    for (i, command) in commands.iter().enumerate() {
        let is_last = commands.len() - 1 == i;
        if let Some(cmd_obj) = ctx.commands.get_command(&command.command) {
            handle_builtin_cmd(command, cmd_obj, is_last, &mut prev_out, ctx)?;
        } else if file::get_command_path(&command.command).is_some() {
            handle_external_cmd(command, is_last, &mut children, &mut prev_out)?;
        } else {
            writer::write_stderr(&format!("invalid command in pipeline: {}\n", command.command), ctx.out)?;
            return Ok(());
        }
    }

    for mut child in children {
        child.wait()?;
    }
    Ok(())
}

fn handle_builtin_cmd(command: &PipelineCommand, cmd_obj: &dyn Command, is_last: bool, prev_out: &mut PrevOut, ctx: &mut ShellContext) -> io::Result<()> {
    ctx.out.pipeout = if is_last {
        None
    } else {
        Some(String::new())
    };

    if let Some(out) = prev_out.builtin.take() {
        ctx.out.pipein = Some(out.to_string());
    } else if let Some(mut out) = prev_out.child.take() {
        let mut buf = String::new();
        out.read_to_string(&mut buf)?;
        ctx.out.pipein = Some(buf);
    }
    cmd_obj.execute(&command.args, ctx)?;

    if !is_last && let Some(out) = ctx.out.pipeout.take() {
        prev_out.builtin = Some(out)
    }
    Ok(())
}

fn handle_external_cmd(command: &PipelineCommand, is_last: bool, children: &mut Vec<Child>, prev_out: &mut PrevOut) -> io::Result<()> {
    let mut cmd = ProcessCommand::new(command.command.clone());
    cmd.args(command.args.clone());
    if let Some(out) = prev_out.child.take() {
        cmd.stdin(out);
    } else if let Some(out) = prev_out.builtin.take() {
        let (reader, mut writer) = io::pipe()?;

        writer.write_all(out.as_bytes())?;
        drop(writer);

        cmd.stdin(reader);
    }

    if !is_last {
        cmd.stdout(Stdio::piped());
    }
    let mut child = cmd.spawn()?;
    
    if !is_last {
        prev_out.child = child.stdout.take();
    }

    children.push(child);
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