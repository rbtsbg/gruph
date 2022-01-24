extern crate gruph;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;

pub fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let path_file: &String = &args[1];
    let file = File::open(path_file)?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(())
}

