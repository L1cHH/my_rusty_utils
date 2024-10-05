use std::fmt::Display;
use std::io;
use std::io::{BufRead};
use my_core_utils::{display_curr_path, execute_command, split_command};
use my_core_utils::commands::command::{Command, ParsingCommand};

fn main() {
    
    loop {
        //Blocking Input for writing Commands
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();

        display_curr_path();

        let mut output = String::new();


        //Reading Input into buffer
        match stdin_lock.read_line(&mut output) {
            Ok(_) => {

                if output.trim() == "exit" {
                    println!("You successfully exit program");
                    break
                }

                output = output
                    .replace(r"\r", "\r")
                    .replace(r"\t", "\t")
                    .replace(r"n", "\n");

                let splitted_command_string = split_command(output);

                if let Err(e) = execute_command(splitted_command_string) {
                    println!("Problem with parsing command: {e:?}")
                };
            },
            Err(read_error) => {
                println!("Problem with reading input into buffer... Error: {read_error:?}")
            }
        }



    }
}
