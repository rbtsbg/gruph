pub mod text {
    
    use std::usize;
    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    
    /// Gets the indices of the next node label in a stanford formatted string.
    ///
    /// Assumes that the node is properly closed, i.e. ends in node separator or white space.
    /// Otherwise will panic.
    ///
    /// # Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `index_start_search` - The start index from where to search the tree for the next node label.
    /// * `separators` - Node separator characters.  
    #[allow(dead_code)]
    pub fn get_next_node_label_indices(
        tree_in: &str,
        index_start_search: usize,
        separators: &[char],
    ) -> Result<(usize, usize), &'static str> {
        let index_first_alphabetic = tree_in[index_start_search..]
            .chars()
            .position(|c| !separators.contains(&c))
            .unwrap();
        let index_next_separator = tree_in[index_start_search + index_first_alphabetic..]
            .chars()
            .position(|c| separators.contains(&c))
            .unwrap();
        return Ok((
            index_first_alphabetic + index_start_search,
            index_next_separator + index_start_search,
        ));
    }
    
    /// Builds a graph from a string representation of a tree.  
    ///
    /// Assumes a tree representation where the nodes are separated as specified by the arguments.
    /// White spaces are not expected. // todo is that wise?
    ///
    /// # Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `node_separator_start` - Character signalling the start of a node.
    /// * `node_separator_end` - Character signalling the end of a node.
    #[allow(dead_code)]
    pub fn build_tree(
        tree_in: &str,
        node_separator_start: &char,
        node_separator_end: &char,
    ) -> Result<Graph<String, ()>, &'static str> {
        let mut res = Graph::<String, ()>::new(); // directed graph
        let mut indices_nodes: Vec<NodeIndex> = Vec::new();
        let mut index_char: usize = 0;
        let chars: Vec<char> = tree_in.chars().collect();
        let separators = [*node_separator_start, *node_separator_end, ' '];
        while index_char < tree_in.len() {
            let ch = chars[index_char];
            if ch == *node_separator_start {
                let (node_index_start, node_index_end) =
                    get_next_node_label_indices(tree_in, index_char, &separators)?;
                let mut node_label: String = String::new();
                for i in node_index_start..=node_index_end {
                    node_label.push(chars[i]);
                }
                indices_nodes.push(res.add_node(node_label.clone()));
                index_char = node_index_end + 1;
            } else if ch == *node_separator_end {
                if indices_nodes.len() > 1 {
                    let target_node = indices_nodes.pop().unwrap();
                    let source_node = indices_nodes.last().unwrap().clone(); // panicked
                    res.add_edge(source_node, target_node, ());
                    index_char = index_char + 1;
                } else {
                    return Ok(res);
                }
            } else if ch.is_whitespace() {
                continue;
            } else {
                println!("Should not encounter {}", ch);
                return Err("Malfomatted string.");
            }
        }
        return Ok(res);
    }

        
        #[test]
        pub fn test_build_tree() {
            let input = "(ROOT(S(NP(PRP)(NN))(ADVP(RB))(VP(VBZ)(S(VP(VBG)(NP(NN)))))))";
            let graph = build_tree(&input, &'(', &')');
            match graph {
                Ok(_g) => println!("tree generated"), // todo test something meaningful
                Err(_) => panic!(),
            }
        }
        
        #[test]
        pub fn test_node_label_indices_correct() {
            let input = "(ROOT(S(NP(PRP)(NN))(ADVP(RB))(VP(VBZ)(S(VP(VBG)(NP(NN)))))))";
            let separators = ['(', ')', ' '];
            let indices = get_next_node_label_indices(&input, 0, &separators);
            match indices {
                Ok((i1, i2)) => assert_eq!((i1, i2), (1, 4)),
                Err(_) => panic!(),
            }
            let indices = get_next_node_label_indices(&input, 0, &separators);
            match indices {
                Ok((i1, i2)) => assert_eq!((i1, i2), (1, 4)),
                Err(_) => panic!(),
            }
        }
        
        

    
}
