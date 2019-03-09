use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    // COMMAND LINE ARGUMENTS L26
    // multiple cli arguments
    for argument in args.iter() {
        println!("{}", argument)
    }

    // single cli argument
    println!("{}", args[1]);

    // WRITING TO A FILE L27
    let mut file = File::create("output.txt").expect("Could not create file!");
    file.write_all(b"Welcome to my file")
        .expect("Could not write to file");
}
