mod io_tests {

    use std::env;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;
    use crate::stanford::*;
    use crate::process::text::build_tree;


    #[test]
    pub fn test_reading_trees_from_text_file() {
        let path_file: &String = &String::from("./resources/trees.txt");
        let path_exists = Path::new(path_file).exists();
        assert_eq!(path_exists, true);
        let file = File::open(path_file);
        let file = match file {
            Ok(f) => f, 
            Err(e) => panic!("Something went wrong with {}", path_file)
        }; 
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut _line: String = match line {
                Ok(l) => l, 
                Err(e) => panic!("Again something with reading a line.")
            };
            let _line: Vec<char> = prettify_stanford_string(&_line[..]);
            let _line: String = _line.iter().collect();
            println!("{}", _line);
            let tree = build_tree(&_line[..], &'(', &')');
            match tree {
                Ok(t) => println!("Build tree w/ {} nodes", t.node_count()), 
                Err(e) => panic!("Something went wrong while building tree")
            }
            // todo: test w/ dijkstra
        }
    }
}
