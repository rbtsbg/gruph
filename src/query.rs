
mod tree {
    use petgraph::graph::DiGraph; 
    use petgraph::algo::{dijkstra};
    use petgraph::algo::dominators::simple_fast;
    use petgraph::graph::{NodeIndex, DefaultIx};

    #[allow(unused_variables, dead_code)]
    pub fn dominates(graph: DiGraph<&str, &str>, node_label_dominater: &str, node_label_dominated: &str) -> Result<bool, String> { 
        return Err(String::from("ToDo: Implement"));
    }

    pub fn generate_test_graph() -> (DiGraph<String, ()>, Vec<NodeIndex<DefaultIx>>){
        let mut g = DiGraph::<String, ()>::new();
        let s = g.add_node(String::from("S"));
        let np = g.add_node(String::from("NP"));
        let det = g.add_node(String::from("DET"));
        let nnp = g.add_node(String::from("NNP"));
        let vp = g.add_node(String::from("VP"));
        let vvp = g.add_node(String::from("VVP"));
        g.extend_with_edges(&[(s,np), (s,vp), (np, det), (np, nnp), (vp, vvp)]);
        return (g, vec![s, np, det, nnp, vp, vvp]); 
    }

    #[test]
    pub fn test_dijsktra(){
        let (g, nodes) = generate_test_graph();
        let dij = dijkstra(&g, nodes[0].into(), None, |_| 1);
        for (k,v) in dij{
            println!("{}:{}", k.index(), v);
        }
        assert!(true);
    }
    
    #[test]
    pub fn test_dominates(){
        let (g, nodes) = generate_test_graph();
        let res = simple_fast(&g, nodes[0].into());
        let dominators = res.dominators(nodes[0]);
        match dominators {
            Some(hm) => {
                for ni in hm {
                    println!("{}", ni.index());
                }
            }, 
            None => {assert!(false)}, 
            }
        }
    }
