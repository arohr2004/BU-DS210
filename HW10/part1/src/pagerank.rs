// pagerank.rs

use crate::graph::Graph;          // Graph struct from the graph module
use std::collections::HashSet;    // HashSet data structure

// simulate a random walk on a graph starting from a given vertex
pub fn simulate_random_walk(graph: &Graph, start_vertex: usize, steps: usize) -> HashSet<usize> {
    // Initialize an empty set to track visited vertices
    let mut visited = HashSet::new();
    // Initialize the current vertex to the start vertex
    let mut current_vertex = start_vertex;

    // Iterate for the specified number of steps
    for _ in 0..steps {
        // Insert the current vertex into the visited set
        visited.insert(current_vertex);
        // Get the neighbors of the current vertex from the graph
        let neighbors = graph.get_neighbors(current_vertex);
        // If there are no neighbors, terminate the walk
        if neighbors.is_empty() {
            break;
        }
        // Choose the next vertex randomly from the neighbors
        current_vertex = *neighbors.iter().next().unwrap(); // Simple random walk
    }

    // Return the list of visited vertices
    visited
}


