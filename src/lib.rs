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
        let g = crate::file_to_graph::chars_to_graph("");
    }

}

mod file_to_graph {
    use petgraph::graph::Graph;
    use petgraph::graph::NodeIndex;

    pub fn chars_to_graph<'a>(line: &str) -> Graph<&'a str, ()> {
        let mut res = Graph::<&'a str, ()>::new(); // directed graph
        let mut nodes: Vec<NodeIndex> = Vec::new(); // collect nodes here
                                                    // todo: collect nodes
                                                    // todo: add to graph
                                                    // todo: add edges

        //        let s = res.add_node("S");
        //        let np = res.add_node("NP");
        //        let vp = res.add_node("VP");
        //        res.extend_with_edges(&[(s, np), (np, vp)]);
        return res;
    }
}

pub fn hello() {
    println!("Hello, world!");
}
