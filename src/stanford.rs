/// From tree and to tree converters.
#[allow(unused_imports)]
pub mod stanford {

    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;
    use std::collections::HashSet;
    use std::usize;

    use crate::core::text::{get_next_node_label_indices, build_tree};

    /// Prettifies a stanford string representation of a syntax tree, i.e. removes white spaces,
    /// encloses leaves in brackets.
    ///
    /// #Arguments
    /// * tree_in - The tree to prettify.
    ///
    #[allow(dead_code)]
    pub fn prettify_stanford_string(tree_in: &str) -> Vec<char> {
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
    
    #[test]
    pub fn test_build_stanford_tree() {
        let mut input: String = String::from("(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))");
        input = prettify_stanford_string(&input).into_iter().collect();
        let graph = build_tree(&input, &'(', &')');
        match graph {
            Ok((g, hm)) => {
                println!("tree generated");
                assert_eq!(g.node_count(), 22);
            }
            Err(_) => panic!("Could not build stanford tree"),
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