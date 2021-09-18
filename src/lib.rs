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

}

/// From tree and to tree converters.
#[allow(unused_imports)]
pub mod converter {

    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use std::collections::HashSet;
    use std::usize;

    /// Gets the indices of the next node label in a stanford formatted string.
    ///
    /// Assumes that the node is properly closed, i.e. ends in node separator or white space.
    /// Otherwise will panic. 
    ///
    /// #Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `index_start_search` - The start index from where to search the tree for the next node label.
    /// * `separators` - Node separator characters.  
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
        return Ok((index_first_alphabetic + index_start_search, 
                   index_next_separator  + index_start_search));
    }

    /// Builds a graph from a string representation of a tree.  
    ///
    /// Assumes a tree representation where the nodes are separated as specified by the arguments.
    /// White spaces are not expected. // todo is that wise?
    ///
    /// #Arguments
    ///
    /// * `tree_in` - Stanford formatted string representation of a tree.
    /// * `node_separator_start` - Character signalling the start of a node. 
    /// * `node_separator_end` - Character signalling the end of a node. 
    pub fn build_tree (
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
                let mut node_label: String = String::new(); 
                for i in node_index_start..=node_index_end{
                    node_label.push(chars[i]);
                } 
                indices_nodes.push(res.add_node(node_label.clone()));
                index_char = node_index_end + 1;
            }
            else if ch == *node_separator_end{
                if indices_nodes.len() > 1 {
                    let target_node = indices_nodes.pop().unwrap();
                    let source_node = indices_nodes.last().unwrap().clone(); // panicked 
                    res.add_edge(source_node, target_node, ());
                    index_char = index_char + 1; 
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
        

    /// Prettifies a stanford string representation of a syntax tree, i.e. removes white spaces,
    /// encloses leaves in brackets. 
    /// 
    /// #Arguments 
    /// * tree_in - The tree to prettify. 
    ///
    #[allow(dead_code)]
    fn prettify_stanford_string(tree_in: &str) -> Vec<char>{
        let chars: Vec<char> = tree_in.chars().collect();
        let mut result: Vec<char> = Vec::new();
        let mut collecting: bool = false; 

        for c in chars{
            // possible leaf
            if c == ' ' {
                result.push(c);
                collecting = true; 
                continue;
            }
            if !collecting {
                result.push(c);
                continue;
            }
            if collecting {
                // not a leaf            
                if c == '(' {
                    collecting = false; 
                    result.push(c);
                    continue;
                }
                     
                // a leaf 
                if c != ')' && *result.last().unwrap() == ' '{
                  result.push('('); 
                  result.push(c);
                  continue;
                }
                else if c != ')'{
                    result.push(c);
                    continue;
                }
                if c == ')'{
                    collecting = false; 
                    result.push(c);
                    result.push(c);
                    continue;
                }
                else {
                    panic!("undefined case: {}", c);
                }
            }
        } 
        result.retain(|&c| c != ' ');
        return result;
    }
    
#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use crate::converter::{get_next_node_label_indices, build_tree, prettify_stanford_string};
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
    
    #[test]
    pub fn test_build_original_tree() {
        let mut input: String = String::from("(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))");
        input = prettify_stanford_string(&input).into_iter().collect();
        let graph = build_tree(&input, &'(', &')'); 
        match graph {
            Ok(_g) => println!("tree generated"),
            Err(_) => panic!(),
        }
    }
  
    #[test]
    pub fn test_prettify_stanford_tree(){
        let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
        let expected = "(ROOT(S(NP(PRP$(My))(NN(dog)))(ADVP(RB(also)))(VP(VBZ(likes))(S(VP(VBG(eating))(NP(NN(sausage))))))(.(.))))";
        let output: String = prettify_stanford_string(&input).into_iter().collect();
        //println!("{}", output);
        assert_eq!(
            expected, output
            );
    }
}

}
