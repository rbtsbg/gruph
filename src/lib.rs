//! <tt>gruph </tt>is a tree regex matcher library, heavily inspired by Tregex.
//!
//! <tt>gruph </tt>provides file-to-tree readers and converters for different types of trees.
//! Trees can then be matched against regular expressions. The regex grammar is adapted to trees.  
mod process;


/// From tree and to tree converters.
#[allow(unused_imports)]
pub mod converter {

    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use std::collections::HashSet;
    use std::usize;

    use crate::process::graph::{get_next_node_label_indices, build_tree};

    /// Prettifies a stanford string representation of a syntax tree, i.e. removes white spaces,
    /// encloses leaves in brackets.
    ///
    /// #Arguments
    /// * tree_in - The tree to prettify.
    ///
    #[allow(dead_code)]
    fn prettify_stanford_string(tree_in: &str) -> Vec<char> {
        let chars: Vec<char> = tree_in.chars().collect();
        let mut result: Vec<char> = Vec::new();
        let mut collecting: bool = false;

        for c in chars {
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
                if c != ')' && *result.last().unwrap() == ' ' {
                    result.push('(');
                    result.push(c);
                    continue;
                } else if c != ')' {
                    result.push(c);
                    continue;
                }
                if c == ')' {
                    collecting = false;
                    result.push(c);
                    result.push(c);
                    continue;
                } else {
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
        use crate::converter::{build_tree, get_next_node_label_indices, prettify_stanford_string};
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
            let input_prettified: String = prettify_stanford_string(&input).into_iter().collect();
            let separators = ['(', ')', ' '];
            let indices = get_next_node_label_indices(&input, 0, &separators);
            match indices {
                Ok((i1, i2)) => assert_eq!((i1, i2), (1, 4)),
                Err(_) => panic!(),
            }
            let indices = get_next_node_label_indices(&input_prettified, 0, &separators);
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
                Ok(_g) => println!("tree generated"), // todo test something meaningful
                Err(_) => panic!(),
            }
        }

        #[test]
        pub fn test_build_original_tree() {
            let mut input: String = String::from("(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))");
            input = prettify_stanford_string(&input).into_iter().collect();
            let graph = build_tree(&input, &'(', &')');
            match graph {
                Ok(g) => {
                    println!("tree generated");
                    assert_eq!(g.node_count(), 22);
                }
                Err(_) => panic!(),
            }
        }

        #[test]
        pub fn test_prettify_stanford_tree() {
            let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
            let expected = "(ROOT(S(NP(PRP$(My))(NN(dog)))(ADVP(RB(also)))(VP(VBZ(likes))(S(VP(VBG(eating))(NP(NN(sausage))))))(.(.))))";
            let output: String = prettify_stanford_string(&input).into_iter().collect();
            assert_eq!(expected, output);
        }
    }
}
