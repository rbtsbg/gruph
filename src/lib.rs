#[cfg(test)]
mod test {
    use petgraph::algo::dijkstra;
    use petgraph::graph::{Graph, NodeIndex, UnGraph};

    // test the dependency.
    #[test]
    pub fn test_ungraph() {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
        let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());
    }

    #[test]
    pub fn test_file_to_graph() {
        let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
        let g = crate::file_to_graph::chars_to_graph(input);
        assert_eq!(g.node_count(), 22);
    }

}

mod file_to_graph {
    use petgraph::graph::Graph;

    pub fn chars_to_graph<'a>(line: &str) -> Graph<String, ()> {
        let mut res = Graph::<String, ()>::new(); // directed graph
        let mut node: String = String::new();
        let mut collecting: bool = false;
        for c in line.chars() {
            if c == '(' {
                continue;
            } else if c == ' ' || c == ')' {
                if !node.is_empty() {
                    res.add_node(node.clone());
                    node = String::new();
                }
                collecting = false;
                continue;
            } else {
                node.push(c);
            }
        }
        return res;
    }
}

pub fn hello() {
    println!("Hello, world!");
}
