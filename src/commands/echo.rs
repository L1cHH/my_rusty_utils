use crate::commands::command::Command;

//echo command
struct EchoCommand {
    payload: String
}

impl EchoCommand {
    pub fn new(input_payload: String) -> EchoCommand {
        Self {
            payload: input_payload
        }
    }
}

impl Command<EchoCommand, 3> for EchoCommand {
    fn available_args(&self) -> [&'static str; 3] {
        ["-n", "-e", "-E"]
    }

    fn command_name(&self) -> &str {
        "echo"
    }

    fn execute_command(&self) {
        todo!()
    }

}