#![feature(iter_advance_by)]
pub mod scanner;
mod token;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn error(line: u32, message: &str) {
    println!("[line: {line}] Error: {message}");
}
