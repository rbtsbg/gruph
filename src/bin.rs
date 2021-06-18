extern crate gruph;

#[allow(unused_imports)]
//use gruph::hello;
use std::collections::HashSet;

#[allow(unused_variables)]
pub fn main() {
    let s = "this is a strirng";
    let separators = ['(', ')'];
    let res = s.chars().find(|c| !separators.contains(c));
    match res {
        Some(c) => println!("{}", c),
        _ => println!("Error"),
    }
}
