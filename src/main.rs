mod commands;
mod hep;
use commands::help::execute;
use std::env::args;

use current_platform::{COMPILED_ON, CURRENT_PLATFORM};


mod utilities {
    pub mod parse_args;
    pub mod resolve_command_function;
    pub mod task_folder;
}

use utilities::parse_args::parse;

use crate::utilities::task_folder;


fn main() {
    println!("Hello< world!");
    println!(
        "Hello, world from {}! I was compiled on {}.",
        CURRENT_PLATFORM, COMPILED_ON
    );
    execute();
    task_folder::create_folder().unwrap();

    let args = parse(&mut args());
    
}
