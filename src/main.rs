use std::fmt::Display;
use std::io;
use std::io::{BufRead};
use my_core_utils::display_curr_path;

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
                println!("{output}");
            },
            Err(read_error) => {
                println!("Problem with reading input into buffer... Error: {read_error:?}")
            }
        }

        if output.trim() == "exit" {
            println!("You successfully exit program");
            break
        }

    }
}
