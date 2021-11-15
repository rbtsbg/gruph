
mod tree {
    use petgraph::graph::{DiGraph};

    #[allow(unused_variables, dead_code)]
    pub fn dominates(graph: DiGraph<&str, &str>, node_label_dominater: &str, node_label_dominated: &str) -> Result<bool, String> { 
        return Err(String::from("ToDo: Implement"));
    }

    #[test]
    pub fn test_dominates(){
        let g = DiGraph::new();
        let node_label_dominator = &"NP"; 
        let node_label_dominated = &"VP"; 
        let dominated = dominates(g, node_label_dominator, node_label_dominated);
        assert!(!dominated.is_err());
    }
}
