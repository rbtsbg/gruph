extern crate gruph;

use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No path found.");
    }
    let path_file: &String = &args[1];
    let path_exists = Path::new(path_file).exists();
    if !path_exists {
        panic!("File {} does not exist.", path_file)
    }
    let file = File::open(path_file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
