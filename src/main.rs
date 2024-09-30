use std::env::current_dir;
use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};
use my_core_utils::display_curr_path;

fn main() {

    loop {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();

        display_curr_path();

        let mut output = String::new();

        match stdin_lock.read_line(&mut output) {
            Ok(_) => {
                println!("{output}");
            },
            Err(read_error) => {
                println!("Problem with reading input into buffer... Error: {read_error:?}")
            }
        }


    }
}
