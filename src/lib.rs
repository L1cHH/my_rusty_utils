use std::env::current_dir;
use std::io;
use std::io::Write;
use std::path::{PathBuf};

mod commands;



pub fn display_curr_path() {
    let curr_dir_path =
        current_dir()
            .unwrap_or_else(|_| String::from("undefined path???").into());

    write!(io::stdout(), "{}>", curr_dir_path.as_path().display()).unwrap();
    io::stdout().flush().unwrap();
}

