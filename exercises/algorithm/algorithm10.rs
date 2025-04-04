use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        let from = String::from(from);
        let to = String::from(to);

        self.add_node(&from);
        self.add_node(&to);

        self.adjacency_table_mutable()
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push((to.clone(), weight));

        self.adjacency_table_mutable()
            .entry(to)
            .or_insert_with(Vec::new)
            .push((from, weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        let node = String::from(node);
        if self.adjacency_table_mutable().contains_key(&node) {
            false
        } else {
            self.adjacency_table_mutable().insert(node, Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32));

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    use crate::HashSet;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge), "Edge {:?} not found", edge);
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        assert!(graph.add_node("a"), "Node 'a' should be added successfully");
        assert!(!graph.add_node("a"), "Node 'a' should not be added again");
        assert!(graph.contains("a"), "Node 'a' should be in the graph");
    }

    #[test]
    fn test_nodes_and_edges() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));

        let nodes: HashSet<&String> = graph.nodes();
        assert_eq!(nodes.len(), 3, "There should be 3 nodes in the graph");
        assert!(nodes.contains(&String::from("a")), "Node 'a' should be in the graph");
        assert!(nodes.contains(&String::from("b")), "Node 'b' should be in the graph");
        assert!(nodes.contains(&String::from("c")), "Node 'c' should be in the graph");

        let edges = graph.edges();
        assert_eq!(edges.len(), 4, "There should be 4 edges in the graph");
        assert!(edges.contains(&(&String::from("a"), &String::from("b"), 5)), "Edge ('a', 'b', 5) should be in the graph");
        assert!(edges.contains(&(&String::from("b"), &String::from("a"), 5)), "Edge ('b', 'a', 5) should be in the graph");
        assert!(edges.contains(&(&String::from("b"), &String::from("c"), 10)), "Edge ('b', 'c', 10) should be in the graph");
        assert!(edges.contains(&(&String::from("c"), &String::from("b"), 10)), "Edge ('c', 'b', 10) should be in the graph");
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("a", "c", 7));
        graph.add_edge(("b", "c", 10));

        let a_neighbours: Vec<_> = graph.adjacency_table().get(&String::from("a")).unwrap().iter()
            .map(|(n, w)| (n.clone(), *w))
            .collect();
        assert!(a_neighbours.contains(&(String::from("b"), 5)));
        assert!(a_neighbours.contains(&(String::from("c"), 7)));

        let b_neighbours: Vec<_> = graph.adjacency_table().get(&String::from("b")).unwrap().iter()
            .map(|(n, w)| (n.clone(), *w))
            .collect();
        assert!(b_neighbours.contains(&(String::from("a"), 5)));
        assert!(b_neighbours.contains(&(String::from("c"), 10)));
    }
}