mod io_tests {

    use crate::process::text::build_tree;
    use crate::stanford::*;
    use std::env;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;

    use petgraph::algo::dijkstra;

    #[test]
    pub fn test_reading_trees_from_text_file() {
        let path_file: &String = &String::from("./resources/trees.txt");
        let path_exists = Path::new(path_file).exists();
        assert_eq!(path_exists, true);
        let file = File::open(path_file);
        let file = match file {
            Ok(f) => f,
            Err(e) => panic!("Something went wrong with {}", path_file),
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut _line: String = match line {
                Ok(l) => l,
                Err(e) => panic!("Again something with reading a line."),
            };
            let _line: Vec<char> = prettify_stanford_string(&_line[..]);
            let _line: String = _line.iter().collect();
            println!("{}", _line);
            let tree = build_tree(&_line[..], &'(', &')');
            let (tr, w2idx) = match tree {
                Ok((t, hm)) => {
                    println!("Build tree w/ {} nodes", t.node_count());
                    (t, hm)
                }
                Err(e) => panic!("Something went wrong while building tree"),
            };
            let start = &String::from("S");
            let root = w2idx.get(start);

            match root {
                Some(indices_root) => {
                    let head_root = indices_root[0];
                    let dij = dijkstra(&tr, head_root, None, |_| 1.0);
                    for (k, v) in dij {
                        let mut weight = tr.node_weight(k);
                        let mut weight = match weight {
                            Some(w) => w,
                            None => panic!("No weight found."),
                        };
                        println!("{} -> {}: {}", start, weight, v);
                    }
                }
                None => {
                    println!("Root not found.");
                }
            };
        }
    }
}
