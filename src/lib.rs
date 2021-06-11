//! <tt>gruph </tt>is a tree regex matcher library, heavily inspired by Tregex.
//!
//! <tt>gruph </tt>provides file-to-tree readers and converters for different types of trees.
//! Trees can then be matched against regular expressions. The regex grammar is adapted to trees.  

/// File handling.
pub mod io {
    //use crate::converter::stanford_string_to_graph;
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
pub mod converter {

    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use std::usize;

    /// Gets the indices of the next node label in a stanford formatted string.
    ///
    /// Assumes that the node is properly closed, i.e. ends in node separator or white space.
    ///
    /// #Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `index_start_search` - The start index from where to search the tree for the next node label.
    /// * `node_separator_start` - The node start separator character.
    /// * `node_separator_end` - The node end separator character.  
    pub fn get_next_node_label_indices(
        tree_in: &str,
        index_start_search: usize,
        node_separator_start: &char,
        node_separator_end: &char,
    ) -> Result<(usize, usize), &'static str> {
        let mut node_label_start: usize = usize::MAX;
        let mut node_label_end: usize = usize::MIN;
        for (i, c) in tree_in.chars().enumerate() {
            if i < index_start_search || c == *node_separator_start {
                continue;
            } else if c == *node_separator_end || c == ' ' {
                if node_label_start != usize::MAX {
                    node_label_end = (i - 1).into();
                    break;
                } else {
                    continue;
                }
            } else {
                if node_label_start != usize::MAX {
                    continue;
                } else {
                    node_label_start = i;
                }
            }
        }
        if node_label_start != usize::MAX {
            if node_label_end >= node_label_start {
                return Ok((node_label_start, node_label_end));
            } else {
                return Err("String not formatted correctly.");
            }
        } else {
            return Err("String not formatted correctly.");
        }
    }

    //    /// Read a stanford formatted string to graph.
    //    /// A stanford formatted string representation of a tree uses `(`,`)` and white spaces as delimiters, e.g.
    //    ///
    //    /// (ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))
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
    //        for c in line.chars() {
    //            if c == *node_seperator_start {
    //                continue;
    //            } else if c == ' ' {
    //                if !node.is_empty() {
    //                    let node_index = res.add_node(node.clone());
    //                    stack.push(node_index);
    //                    // add edge
    //                    node = String::new();
    //                }
    //                continue;
    //            } else if c == *node_seperator_end {/
    //                if !node.is_empty() {
    //                    let node_index = res.add_node(node.clone());
    //                    stack.push(node_index);
    //                    node = String::new();
    //                }
    //            } else {
    //                node.push(c);
    //            }
    //        }
    //        return res;
    //    }
}

#[cfg(test)]
mod test {
    use crate::converter::get_next_node_label_indices;
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
        let indices = get_next_node_label_indices(&input, 0, &'(', &')');
        match indices {
            Ok((i1, i2)) => assert_eq!((i1, i2), (1, 4)),
            Err(_) => assert_eq!(true, false),
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

pub fn hello() {
    println!("Hello, world!");
}
