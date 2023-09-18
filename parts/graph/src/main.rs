use petgraph::Directed;
use petgraph::graphmap::DiGraphMap;
use petgraph::graph::{Graph, DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Oscillator<'a> {
    private_value: [i32; 5],
    id: &'a str,
}

impl<'a> Oscillator<'a> {
    fn new(value: [i32; 5], id: &'a str) -> Self {
        Oscillator {
            private_value: value,
            id,
        }
    }

    fn shared_function(&self) {
        println!("This is shared function of Oscillator");
    }

    fn get_value(&self) -> [i32; 5] {
        self.private_value
    }

    fn get_id(&self) -> &'a str {
        self.id
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.private_value = new_value;
    }
}

#[derive(Clone, Copy)]
struct ADSR<'a> {
    private_value: [i32; 5],
    id: &'a str,
}

impl<'a> ADSR<'a> {
    fn new(value: [i32; 5], id: &'a str) -> Self {
        ADSR {
            private_value: value,
            id,
        }
    }

    fn shared_function(&self) {
        println!("This is shared function of ADSR");
    }

    fn get_value(&self) -> [i32; 5] {
        self.private_value
    }

    fn get_id(&self) -> &'a str {
        self.id
    }


    fn set_value(&mut self, new_value: [i32; 5]) {
        self.private_value = new_value;
    }
}

// Define a trait that specifies the shared function
trait AudioModule {
    fn shared_function(&self);
    fn get_value(&self) -> [i32; 5];
    fn set_value(&mut self, new_value: [i32; 5]);
}

// Implement the trait for your structs
impl<'a> AudioModule for Oscillator<'a> {
    fn shared_function(&self) {
        self.shared_function();
    }

    fn get_value(&self) -> [i32; 5] {
        self.get_value()
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.set_value(new_value);
    }
}

impl<'a> AudioModule for ADSR<'a> {
    fn shared_function(&self) {
        self.shared_function();
    }

    fn get_value(&self) -> [i32; 5] {
        self.get_value()
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.set_value(new_value);
    }
}

fn find_nodes_without_parents<'a>(graph: DiGraphMap<&'a str, i32>) -> Vec<&'a str> {
    let mut nodes_without_parents: Vec<&'a str> = Vec::new();

    // Iterate over all nodes in the graph
    for node_index in graph.nodes().into_iter() {
        // Check if the node has no incoming edges (i.e., no parents)
        if !graph.neighbors_directed(node_index, petgraph::Direction::Incoming).any(|_| true) {
            nodes_without_parents.push(node_index);
        }
    }

    nodes_without_parents
}

fn get_node_index(graph: Box<DiGraphMap<&str, i32>>, target_node: &str) -> Option<NodeIndex<u32>> {
    let graph: Graph<&str, i32, Directed> = graph.into_graph();
    for (node_index, node) in graph.node_indices().enumerate() {
        if graph[node] == target_node {
            return Some((node_index as u32).into());
        }
    }
    None
}

fn find_all_paths_by_content<'a>(
    graph: &DiGraph<&'a str, i32>,
    start_content: &'a str,
    end_content: &'a str,
) -> Vec<Vec<&'a str>> {
    // Create a map from content to node index
    let mut content_to_node_index: HashMap<&'a str, NodeIndex> = HashMap::new();
    for (node_index, node_content) in graph.node_indices().zip(graph.node_weights()) {
        content_to_node_index.insert(node_content, node_index);
    }

    // Get the start and end node indices
    let start_index = content_to_node_index.get(&start_content).unwrap();
    let end_index = content_to_node_index.get(&end_content).unwrap();

    // Define a function to perform DFS with backtracking
    fn dfs_paths<'a>(
        graph: &DiGraph<&'a str, i32>,
        current_node: NodeIndex,
        end_node: NodeIndex,
        visited: &mut Vec<bool>,
        current_path: &mut Vec<&'a str>,
        all_paths: &mut Vec<Vec<&'a str>>,
    ) {
        visited[current_node.index()] = true;
        current_path.push(graph.node_weight(current_node).unwrap());

        if current_node == end_node {
            all_paths.push(current_path.clone());
        } else {
            for neighbor in graph.neighbors(current_node) {
                if !visited[neighbor.index()] {
                    dfs_paths(graph, neighbor, end_node, visited, current_path, all_paths);
                }
            }
        }

        visited[current_node.index()] = false;
        current_path.pop();
    }

    // Initialize data structures and perform DFS with backtracking
    let mut dfs = Dfs::new(graph, *start_index);
    let mut all_paths: Vec<Vec<&'a str>> = Vec::new();
    let mut visited = vec![false; graph.node_count()];
    let mut current_path: Vec<&'a str> = Vec::new();

    while let Some(node_index) = dfs.next(graph) {
        dfs_paths(graph, node_index, *end_index, &mut visited, &mut current_path, &mut all_paths);
    }

    all_paths
}

fn main() {
    //Create all the existing AudioModules
    let mut g  = DiGraphMap::new();
    let mut structs: Vec<Box<dyn AudioModule>> = vec![];
    let modules = ["A1", "O1", "F1", "A2", "O2", "F2"];
    let connections: Vec<[&str;2]> = vec![["A1", "O1"], ["O1", "F1"],["F1", "Out"],["A1", "O2"],["O2", "F2"],["F2" ,"Out"]];
    //Connect them according to a... json file?

    for connection in connections {
        g.add_edge(connection[0],connection[1], 1);

        let id1 = connection[0];
        let struct1 = Oscillator::new([0; 5], id1);
        structs.push(Box::new(struct1));
        println!("Oscillator<'a> ID: {}", struct1.get_id());

        let id2 = connection[1];
        let struct2 = ADSR::new([0; 5], id2);
        structs.push(Box::new(struct2));
        println!("ADSR<'a> ID: {}", struct2.get_id());
    }


    let all_paths = find_all_paths_by_content(&g.into_graph(),
              "A1",
            "Out");

    let mut sets_of_paths: Vec<HashSet<&str>> = vec![];

    for path in all_paths {
        let set: HashSet<&str> = HashSet::from_iter(path.iter().cloned());
        sets_of_paths.push(set);
    }

    for s in structs.iter_mut() {
        //s.shared_function();
        //println!("Value: {:?}", s.get_value());
        s.set_value([0; 5]);
        //s.shared_function();
    }


}
