mod file_to_graph {
    use petgraph::graph::Graph;
    use std::fs::File;
    use std::io::Error as IOE;
    use std::io::Read;

    #[allow(dead_code)]
    pub fn file_to_graph(path: &str) -> Result<Vec<Graph<String, ()>>, IOE> {
        let res: Vec<Graph<String, ()>> = Vec::new();

        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        for (idx, line) in content.lines().enumerate() {
            let g = line_to_graph(&line, &'(', &')');
            println!("line {} size: {}", idx, g.node_count());
        }

        return Ok(res);
    }

    pub fn line_to_graph(
        line: &str,
        node_seperator_start: &char,
        node_seperator_end: &char,
    ) -> Graph<String, ()> {
        let mut res = Graph::<String, ()>::new(); // directed graph
        let mut node: String = String::new();
        for c in line.chars() {
            if c == *node_seperator_start {
                continue;
            } else if c == ' ' || c == *node_seperator_end {
                if !node.is_empty() {
                    res.add_node(node.clone());
                    node = String::new();
                }
                continue;
            } else {
                node.push(c);
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
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
    pub fn test_line_conversion_to_graph() {
        let input = "(ROOT (S (NP (PRP$ My) (NN dog)) (ADVP (RB also)) (VP (VBZ likes) (S (VP (VBG eating) (NP (NN sausage))))) (. .)))";
        let g = crate::file_to_graph::line_to_graph(input, &'(', &')');
        assert_eq!(g.node_count(), 22);
    }

    #[test]
    pub fn test_file_to_graph() {
        let path: &str = "./resources/trees.txt";
        #[allow(unused_variables)]
        let res = crate::file_to_graph::file_to_graph(path);
    }

}

pub fn hello() {
    println!("Hello, world!");
}
