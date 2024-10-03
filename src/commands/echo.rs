use crate::commands::command::{Command, ParsingCommand};
use crate::commands::error::ParsingCommandError;

//echo command
pub struct EchoCommand {
    payload: String
}

impl EchoCommand {
    pub fn new(input_payload: String) -> EchoCommand {
        Self {
            payload: input_payload
        }
    }
}

impl ParsingCommand<EchoCommand, 3> for EchoCommand {
    fn parse_command(mut input: Vec<String>) -> Result<EchoCommand, ParsingCommandError> {

        let available_args = EchoCommand::available_args();

        //Find all args for current command
        let provided_args: Vec<&String> = input.iter().filter(|&arg| arg.starts_with("-")).collect();

        //Command without args
        match provided_args.is_empty() {
            false => {
                if provided_args.iter().all(|arg| {
                    available_args.contains(&arg.as_str())
                }) {
                    todo!()
                } else {
                    return Err(ParsingCommandError::IncorrectArgs)
                }
            },
            true => {
                input.remove(0);
                Ok(EchoCommand::new(input.pop().unwrap()))
            }
        }

    }
}

impl Command<3> for EchoCommand {
    fn command_name(&self) -> &str {
        "echo"
    }

    fn available_args() -> [&'static str; 3] {
        ["-n", "-e", "-E"]
    }

    fn execute_command(&self) {
        let output = &self.payload;
        println!("Result echo's command: {output}")
    }

}