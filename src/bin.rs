use std::{env, fs, process::exit};

use text_io::read;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    }

    if let Some(path) = args.get(1) {
        run_file(path);
        return;
    }

    run_prompt();
}

fn run_file(path: &String) {
    fs::read_to_string(path).expect("Should have been able to read the file");
    todo!();
    // if let Err(()) = run() {
    //     exit(65);
    // }
}

fn run_prompt() {
    loop {
        let line: String = read!();
        if !line.is_empty() {
            if let Err(()) = run(line) {
                exit(65);
            }
        }
    }
}

fn run(value: String) -> Result<(), ()> {
    // 0. Scan the source code
    // 1. Create tokens
    // 2. print created tokens
    todo!();
}
