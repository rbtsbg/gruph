extern crate gruph;

use std::fs;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let path_file: &String = &args[1];
    println!("Input file: {}", path_file);  

    let contents = fs::read_to_string(path_file).expect("Something went wrong reading the file.");
    println!("Contents: {}", contents);
}
