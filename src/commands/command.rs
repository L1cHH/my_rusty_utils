pub trait Command<C, const N: usize> {
    fn command_name(&self) -> &str;
    fn available_args(&self) -> [&'static str; N];
    fn execute_command(&self);
    fn from_input_string(input: Vec<String>) -> Result<C, ()> {
        let command_name = match input.first() {
            Some(name) => name,
            None => return Err(println!("No one command was provided..."))
        };
        todo!()
    }
}