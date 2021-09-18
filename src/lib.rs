//! <tt>gruph </tt>is a tree regex matcher library, heavily inspired by Tregex.
//!
//! <tt>gruph </tt>provides file-to-tree readers and converters for different types of trees.
//! Trees can then be matched against regular expressions. The regex grammar is adapted to trees.  

/// File handling.
#[allow(unused_imports)]
pub mod io {
    use petgraph::graph::Graph;
    use std::fs::File;
    use std::io::Error as IOE;
    use std::io::Read;

    //    #[allow(dead_code)]
    //    /// Reads a graphs from a file, line by line.
    //    ///
    //    /// # Arguments
    //    ///
    //    /// * `path` - File path to the file containing the string graphs.
    //    ///
    //    /// # Examples
    //    ///
    //    /// ```
    //    /// use petgraph::graph::Graph;
    //    /// use std::io::Error as IOE;
    //    /// use gruph::io::file_to_graph;
    //    /// let path: &str = &"path/to/input/file.txt";
    //    /// let graphs: Result<Vec<Graph<String, ()>>, IOE> = file_to_graph(path);
    //    /// match graphs {
    //    ///   Ok(g) => println!("Loaded graphs."),
    //    ///   Err(e) => println!("Invalid path."),
    //    /// }
    //    /// ```
    //    /// This module reads graphs from file.`
    //    pub fn file_to_graph(path: &str) -> Result<Vec<Graph<String, ()>>, IOE> {
    //        let res: Vec<Graph<String, ()>> = Vec::new();
    //
    //        let mut file = File::open(path)?;
    //        let mut content = String::new();
    //        file.read_to_string(&mut content)?;
    //
    //        for (idx, line) in content.lines().enumerate() {
    //            let g = stanford_string_to_graph(&line, &'(', &')');
    //            println!("line {} size: {}", idx, g.node_count());
    //        }
    //
    //        return Ok(res);
    //    }
}

/// From tree and to tree converters.
#[allow(unused_imports)]
pub mod converter {

    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use std::collections::HashSet;
    use std::usize;

    //static alphabet: [char; 26] = [('a'..='z').collect::Vec<char>()];

    /// Gets the indices of the next node label in a stanford formatted string.
    ///
    /// Assumes that the node is properly closed, i.e. ends in node separator or white space.
    /// Otherwise will panic. 
    ///
    /// #Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `index_start_search` - The start index from where to search the tree for the next node label.
    /// * `node_separator_start` - The node start separator character.
    /// * `node_separator_end` - The node end separator character.  
    /// let input = "(ROOT(S(NP(PRP)(NN))(ADVP(RB))(VP(VBZ)(S(VP(VBG)(NP(NN)))))))";
    pub fn get_next_node_label_indices(
        tree_in: &str,
        index_start_search: usize,
        //node_separator_start: &char,
        //node_separator_end: &char,
        separators: &[char], 
    ) -> Result<(usize, usize), &'static str> {
        //let separators = [*node_separator_start, *node_separator_end, ' ']; // fixme: whitespace hard coded
        let index_first_alphabetic = tree_in[index_start_search..]
            .chars()
            .position(|c| !separators.contains(&c))
            .unwrap();
        let index_next_separator = tree_in[index_start_search + index_first_alphabetic..]
            .chars()
            .position(|c| separators.contains(&c))
            .unwrap();
        return Ok((index_first_alphabetic + index_start_search, 
                   index_next_separator  + index_start_search));
    }

    pub fn build_tree(
        tree_in: &str, 
        node_separator_start: &char, 
        node_separator_end: &char, 
        ) -> Result<Graph<String, ()>, &'static str>{
        let mut res = Graph::<String, ()>::new(); // directed graph
        let mut indices_nodes: Vec<NodeIndex> = Vec::new();
        let mut index_char: usize = 0; 
        let chars: Vec<char> = tree_in.chars().collect();
        let separators = [*node_separator_start, *node_separator_end, ' '];
        while index_char < tree_in.len(){
            let ch = chars[index_char]; 
            if ch == *node_separator_start {
                let (node_index_start, node_index_end) = get_next_node_label_indices(
                    tree_in,
                    index_char,
                    &separators)?; 
//                    node_separator_start,
//                    node_separator_end)?;
                //println!("{}, {}", node_index_start, node_index_end);
                let mut node_label: String = String::new(); 
                for i in node_index_start..=node_index_end{
                    node_label.push(chars[i]);
                } 
                indices_nodes.push(res.add_node(node_label.clone()));
                println!("{}", node_label);
                index_char = node_index_end + 1;
                //println!("index char: {}", index_char);
                // copy from tree_in to string jkk
            }
            else if ch == *node_separator_end{
                if indices_nodes.len() > 1 {
                    let target_node = indices_nodes.pop().unwrap();
                    let source_node = indices_nodes.last().unwrap().clone(); // panicked 
                    res.add_edge(source_node, target_node, ());
                    index_char = index_char + 1; 
                    //println!("index char end {}", index_char);
                } else {
                    return Ok(res)
                }
            }
            else if ch.is_whitespace(){
                continue;
            }
            else {
                println!("Should not encounter {}", ch);
                return Err("Malfomatted string.")
            }
        }
        return Ok(res);
    }


    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0e33616e0daaff83e86e28623a63175b
    //let alphabetic = ('a'..='z').collect::<HashSet<char>>();
    //let separators = [*node_separator_start, *node_separator_end]
    //    .iter()
    //    .cloned()
    //    .collect::<HashSet<char>>();
    //let non_separators = alphabetic.difference(&separators);}
    //        let index_node_label_start: Option<usize> =
    //            tree_in[index_start_search..].find(char::is_alphabetic); // Works
    //        let index_node_label_end = match index_node_label_start {
    //            Some(u) => tree_in[u..].find(&[*node_separator_start, *node_separator_end, ' '][..]),
    //            None => None,
    //        };
    //        match (index_node_label_start, index_node_label_end) {
    //            (Some(start), Some(end)) => return Ok((start, end)),
    //            _ => Err("Malformatted string."),
    //        }
//})

//    /// Read a stanford formatted string to graph.
//    /// A stanford formatted string representation of a tree uses `(`,`)` and white spaces as delimiters, e.g.
//    ///
//    /// (ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB al)so)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))
//
//    /// R -> S -> N -> P -> M
//
//        fn read_tree(
//            tree_in: &str,
//            tree_out: Graph<String, ()>,
//            node_separator_start: &char,
//            node_separator_end: &char,
//        ) -> Option<NodeIndex> {
//            let mut res_index: Option<NodeIndex> = None;
//            for (i, c) in tree_in.chars().enumerate() {
//                if c == *node_separator_start {
//                    match res_index {
//                        Some(idx) => {
//                            read_tree(&tree_in[i..], node_separator_start, node_separator_end);
//                        }
//                        None => {}
//                    }
//                }
//            }
//            return res_index;
//        }

//    pub fn stanford_string_to_graph(
//        line: &str,
//        node_seperator_start: &char,
//        node_seperator_end: &char,
//    ) -> Graph<String, ()> {
//        let mut res = Graph::<String, ()>::new(); // directed graph
//        let mut node: String = String::new();
//        let mut stack: Vec<NodeIndex> = Vec::new();
//        let mut is_leaf: bool = false;
//        for c in line.chars() {)
//            if c == *node_seperator_start {
//                continue;
//            } else if c == ' ' {)
//                if !node.is_empty() {
//                    let node_index = res.add_node(node.clone());
//                    stack.push(node_index);
//                    // add edge)
//                    node = String::new();
//                }
//                continue;
//            } else if c == *node_seperator_end {/
//                if !node.is_empty() {
//                    let node_index = res.add_node(node.clone());
//                    stack.push(node_index);
//                    node = String::new();)
//                }
//            } else {
//                node.push(c);
//            }
//        }
//        return res;
//    }
//}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use crate::converter::{get_next_node_label_indices, build_tree};
    use petgraph::algo::dijkstra;
    use petgraph::graph::{NodeIndex, UnGraph};

    // test the dependency.
    #[test]
    pub fn test_ungraph_is_created_from_scratch() {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
        let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());
    }

    #[test]
    pub fn test_node_label_indices_correct() {
        let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
        let separators = ['(', ')', ' '];
        let indices = get_next_node_label_indices(&input, 0, &separators);
        match indices {
            Ok((i1, i2)) => assert_eq!((i1, i2), (1, 4)),
            Err(_) => panic!(),
        }
    }
    
    #[test]
    pub fn test_build_tree() {
        let input = "(ROOT(S(NP(PRP)(NN))(ADVP(RB))(VP(VBZ)(S(VP(VBG)(NP(NN)))))))";
        let graph = build_tree(&input, &'(', &')'); 
        match graph {
            Ok(_g) => println!("tree generated"),
            Err(_) => panic!(),
        }
    }
}

//    #[test]
//    pub fn test_line_conversion_to_graph() {
//        let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
//        let g = crate::converter::stanford_string_to_graph(input, &'(', &')');
//        assert_eq!(g.node_count(), 22);
//    }
//
//    #[test]
//    pub fn test_file_to_graph() {
//        let path: &str = "./resources/trees.txt";
//        #[allow(unused_variables)]
//        let res = crate::io::file_to_graph(path);
//    }
//}


}
