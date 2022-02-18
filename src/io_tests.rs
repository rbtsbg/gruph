mod io_tests {

    use crate::core::text::build_tree;
    use crate::stanford::stanford::*;
    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    use std::path::Path;

    use petgraph::algo::dijkstra;
    use petgraph::graph::DiGraph;
    use petgraph::graph::NodeIndex;

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
                Err(e) => panic!("Something went wrong."),
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
            let start = &String::from("VP");
            let root = w2idx.get(start);

            let vp_dom_np = dominates(&tr, &w2idx, &String::from("VP"), &String::from("NP"));
            println!("VP dominates NP: {}", vp_dom_np);
            
            let np_dom_dt = dominates(&tr, &w2idx, &String::from("NP"), &String::from("DT"));
            println!("NP dominates DT: {}", np_dom_dt);

            let vp_dom_dt = dominates(&tr, &w2idx, &String::from("VP"), &String::from("DT"));
            println!("VP dominates DT: {}", vp_dom_dt);
            
            let root_dom_vp = dominates(&tr, &w2idx, &String::from("ROOT"), &String::from("VP"));
            println!("ROOT dominates VP: {}", root_dom_vp);
            
            let root_dom_np = dominates(&tr, &w2idx, &String::from("ROOT"), &String::from("NP"));
            println!("ROOT dominates NP: {}", root_dom_np);
            
            let vp_dom_root = dominates(&tr, &w2idx, &String::from("VP"), &String::from("ROOT"));
            println!("VP dominates ROOT: {}", vp_dom_root);
            
            let np_dom_root = dominates(&tr, &w2idx, &String::from("NP"), &String::from("ROOT"));
            println!("NP dominates ROOT: {}", np_dom_root);




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
                    println!("Start not found."); // todo: start implementing match function from here
                }
            };
        }
    }

    pub fn dominates(
        tree: &DiGraph<String, ()>,
        weight_to_index: &HashMap<String, Vec::<NodeIndex>>,
        dominator: &String,
        dominated: &String
    ) -> bool {
        let mut start = weight_to_index[dominator][0];
        let paths = dijkstra(tree, start, None, |_| 1);
        for (k, v) in paths {
            let mut weight = tree.node_weight(k);
            match weight {
                Some(w) => {
                    if w == dominated {
                        return true
                    }
                },
                None => continue
            }
        }
        return false;
    }
}
