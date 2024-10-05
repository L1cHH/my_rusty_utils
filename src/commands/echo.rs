use std::io;
use std::io::Write;
use crate::commands::command::{Command, ParsingCommand};
use crate::commands::error::ParsingCommandError;

//echo command
pub struct EchoCommand<'args> {
    payload: Vec<&'args str>,
    provided_args: Option<Vec<&'args str>>
}

impl <'args> EchoCommand<'args> {
    pub fn with_args(payload: Vec<&'args str>, provided_args: Vec<&'args str>) -> EchoCommand<'args> {
        Self {
            payload,
            provided_args: Some(provided_args)
        }
    }

    pub fn with_empty_args(payload: Vec<&'args str>) -> EchoCommand {
        Self {
            payload,
            provided_args: None
        }
    }

}

impl <'args> ParsingCommand<'args, EchoCommand<'args>, 3> for EchoCommand<'_> {
    fn parse_command(input: &'args mut Vec<String>) -> Result<EchoCommand, ParsingCommandError> {

        //Only command name was provided
        if input.is_empty() {
            return Err(ParsingCommandError::OnlyCommandProvided);
        };

        //Vec with Args
        let args: Vec<&str> = input
            .iter()
            .map(|arg| arg.as_str())
            .filter(|&arg| arg.starts_with("-"))
            .collect();

        //Vec with payload (in this case are strings to print)
        let payload: Vec<&str> = input
            .iter()
            .map(|arg| arg.as_str())
            .filter(|&arg| !arg.starts_with("-"))
            .collect();

        match args.is_empty() {
            true => {
                Ok(EchoCommand::with_empty_args(payload))
            },

            false => {
                if !payload.is_empty() {

                    let available_args = EchoCommand::available_args();

                   if args.iter().all(|arg| available_args.contains(arg)) {
                       Ok(EchoCommand::with_args(payload, args))
                   } else {
                       Err(ParsingCommandError::IncorrectArgs)
                   }
                } else {
                    //if there are args but no one payload
                    return Err(ParsingCommandError::IncorrectArgs)
                }


            }
        }
    }
}

impl Command<3> for EchoCommand<'_> {
    fn command_name(&self) -> &str {
        "echo"
    }

    fn available_args() -> [&'static str; 3] {
        ["-n", "-e", "-E"]
    }

    fn execute_command(&self) {
        match &self.provided_args {
            Some(args) => {
                if args.contains(&"-n") {
                    let output = &self.payload;

                    for &string in output.iter() {
                        if string == *output.last().unwrap() {
                            write!(io::stdout(), "{string}").unwrap();
                        } else {
                            write!(io::stdout(), "{string} ").unwrap();
                        }

                        io::stdout().flush().unwrap();
                    }
                } else if args.contains(&"-n") && args.contains(&"-E") {
                    let output = &self.payload;

                    for &string in output.iter() {
                        if string == *output.last().unwrap() {
                            write!(io::stdout(), r"{string}").unwrap();
                        } else {
                            write!(io::stdout(), r"{string} ").unwrap();
                        }

                        io::stdout().flush().unwrap();
                    }
                } else if args.contains(&"-n") && args.contains(&"-e") {
                    let output = &self.payload;

                    for &string in output.iter() {
                        if string == *output.last().unwrap() {
                            write!(io::stdout(), "{string}").unwrap();
                        } else {
                            write!(io::stdout(), "{string} ").unwrap();
                        }

                        io::stdout().flush().unwrap();
                    }
                }

            }
            None => {
                let output = &self.payload;

                for &string in output.iter() {
                    if string == *output.last().unwrap() {
                        writeln!(io::stdout(), "{string}").unwrap();
                    } else {
                        writeln!(io::stdout(), "{string} ").unwrap();
                    }

                    io::stdout().flush().unwrap();
                }

            }
        }


    }

}