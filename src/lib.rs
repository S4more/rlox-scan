#![feature(iter_advance_by)]
#![feature(let_chains)]
#![feature(trait_alias)]
pub mod scanner;
mod token;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn error(line: u32, message: &str) {
    println!("[line: {line}] Error: {message}");
}
