mod io_tests {

    use std::env;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;
    use crate::stanford::*;
    use crate::process::text::build_tree;

    use petgraph::algo::dijkstra;

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
            let (tr, w2idx) = match tree {
                Ok((t, hm)) => {
                    println!("Build tree w/ {} nodes", t.node_count()); 
                    (t, hm)},
                Err(e) => panic!("Something went wrong while building tree")
            };
            let root = w2idx.get(&String::from("ROOT"));
            let np = w2idx.get(&String::from("NP"));
            match root {
                Some(indices_root) => {
                    match np {
                        Some(indices_np) => {
                           let head_root = indices_root[0];
                           let dij = dijkstra(&tr, head_root, None, |_| 1.0);
                           for (k,v) in dij {
                               println!("{}", v);
                           }
                        }, 
                        None => {
                            println!("Did not encounter NP");
                        }
                        // todo c
                    }
                },
                None => {
                    println!("Root not found.")
                }
            }
            // todo: test w/ dijkstra
        }
    }
}
