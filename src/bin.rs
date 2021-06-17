extern crate gruph;

#[allow(unused_imports)]
//use gruph::hello;
use std::collections::HashSet;

pub fn main() {
    println!("hello, world!");
    let line = "(&NP (VP (V is)))";
    let alphabetic = ('A'..='Z').collect::<HashSet<char>>();
    let abc = ['a', 'b', 'c'].iter().cloned().collect::<HashSet<char>>();
    let diff: Vec<char> = alphabetic.difference(&abc).cloned().collect();
    let index_node_label_start: Option<usize> = line.find(&diff[..]);
    match index_node_label_start {
        Some(idx) => println!("{}", idx),
        None => println!("Error"),
    }
    for item in diff {
        println!("item: {}", item);
    }
}
