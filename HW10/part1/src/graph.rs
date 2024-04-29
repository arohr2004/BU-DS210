// graph.rs
// Import the HashMap data structure 
use std::collections::HashMap;

// Define a struct representing a graph
pub struct Graph {
    vertices: Vec<usize>,                   // Vector to store vertex IDs
    adjacency_list: HashMap<usize, Vec<usize>>,  // HashMap to store adjacency lists
}

impl Graph { // Constructor function to create a new graph instance
    pub fn new() -> Self {
        // Initialize an empty graph with an empty vertices vector and an empty adjacency list
        Graph {
            vertices: Vec::new(),
            adjacency_list: HashMap::new(),
        }
    }

    // add a vertex to the graph
    pub fn add_vertex(&mut self, vertex: usize) {
        // Check if the vertex is not already in the graph
        if !self.vertices.contains(&vertex) {
            // Add the vertex to the vertices vector
            self.vertices.push(vertex);
            // Insert an empty list for the vertex in the adjacency list
            self.adjacency_list.insert(vertex, Vec::new());
        }
    }

    // add an edge between two vertices in the graph
    pub fn add_edge(&mut self, from: usize, to: usize) {
        // Ensure both vertices exist in the graph
        self.add_vertex(from);
        self.add_vertex(to);
        // Add the 'to' vertex to the adjacency list of the 'from' vertex
        self.adjacency_list.entry(from).or_insert(Vec::new()).push(to);
    }

    // find the count of vertices in the graph
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()  // Return the length of the vertices vector
    }

    // find the neighbors of a vertex in the graph
    pub fn get_neighbors(&self, vertex: usize) -> Vec<usize> {
        // Check if the vertex exists in the adjacency list
        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            neighbors.clone()  // Return a clone of the list of neighbors
        } else {
            Vec::new()         // Return an empty vector if the vertex has no neighbors
        }
    }
}



