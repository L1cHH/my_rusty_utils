use std::env::current_dir;
use std::io;
use std::io::Write;
use crate::commands::command::{Command, ParsingCommand};
use crate::commands::echo::EchoCommand;
use crate::commands::error::ParsingCommandError;

pub mod commands;



pub fn display_curr_path() {
    let curr_dir_path =
        current_dir()
            .unwrap_or_else(|_| String::from("undefined path???").into());

    write!(io::stdout(), "{}>", curr_dir_path.as_path().display()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn split_command(raw_command: String) -> Vec<String> {
    let raw_command: Vec<&str> = raw_command.split(' ').collect();
    raw_command.into_iter().map(|command_element | command_element.to_string()).collect()
}

pub fn execute_command(command_input: Vec<String>) -> Result<(), ParsingCommandError> {
    let command_name = command_input.first().unwrap().as_str();

    let command = match command_name {
        "echo" => EchoCommand::parse_command(command_input)?,
        _ => return Err(ParsingCommandError::IncorrectCommandName)
    };

    command.execute_command();

    Ok(())
}

