use crate::commands::echo::EchoCommand;
use crate::commands::error::ParsingCommandError;

pub trait Command<const N: usize> {
    fn command_name(&self) -> &str;
    fn available_args() -> [&'static str; N];
    fn execute_command(&self);
}

pub trait ParsingCommand<'args, C, const N: usize>
where
    C: Command<N>,
{
    fn parse_command(raw_command: &'args mut Vec<String>) -> Result<C, ParsingCommandError>;

}
