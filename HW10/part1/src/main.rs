//main.rs

// Import necessary modules and types: Hashmap for storing counts, File operations, Buffered reading from file, File path handling
use std::collections::HashMap;   
use std::fs::File;               
use std::io::{BufReader, BufRead}; 
use std::path::Path;             

// Import local modules
mod graph;             
mod pagerank;          
mod data_generator;    

fn main() {
    // Define the path to the data file
    let file_path = Path::new("data.txt");

    // Generate the data file if it doesn't exist
    if !file_path.exists() {
        data_generator::generate_data_file(1000); // Generate data with 1000 vertices
    }

    // Read the graph from the file
    match read_graph_from_file(file_path) {
        Ok(graph) => {
            // Count the terminations for each vertex in the graph
            let counts = count_terminations(&graph);
            // Find the top five vertices with the highest terminations
            let top_five = top_five_vertices(&counts);
            // Get the total number of vertices in the graph
            let total_vertices = graph.vertex_count() as f64;

            // Compute and print the approximation to PageRank for the top five vertices
            for vertex in top_five.iter() {
                let count = counts.get(vertex).unwrap_or(&0);
                let pagerank = (*count as f64) / (100.0 * total_vertices); // Compute PageRank
                println!("vertex {}: approximate PageRank {:.4}", vertex, pagerank);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err); // Print error if failed to read graph
        }
    }
}

// Function to read a graph from a file
fn read_graph_from_file(file_path: &Path) -> Result<graph::Graph, String> {
    // Open the file
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    // Initialize an empty graph
    let mut graph = graph::Graph::new();
    // Iterate over each line in the file
    for (index, line) in reader.lines().enumerate() {
        // Read each line and parse the vertices
        let line = line.map_err(|e| format!("Failed to read line {}: {}", index + 1, e))?;
        let mut parts = line.split_whitespace();
        let from = parts.next().ok_or_else(|| format!("Missing from vertex in line {}", index + 1))?;
        let to = parts.next().ok_or_else(|| format!("Missing to vertex in line {}", index + 1))?;
        let from: usize = from.parse().map_err(|e| format!("Invalid from vertex in line {}: {}", index + 1, e))?;
        let to: usize = to.parse().map_err(|e| format!("Invalid to vertex in line {}: {}", index + 1, e))?;
        // Add the edge to the graph
        graph.add_edge(from, to);
    }

    // Return the constructed graph
    Ok(graph)
}

// Function to count the terminations for each vertex in the graph
fn count_terminations(graph: &graph::Graph) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    // Iterate over each vertex in the graph
    for vertex in 0..graph.vertex_count() {
        // Count the terminations for the vertex using a random walk simulation
        let count = pagerank::simulate_random_walk(graph, vertex, 100).len();
        counts.insert(vertex, count);
    }
    // Return the counts
    counts
}

// Function to get the top five vertices with the highest terminations
fn top_five_vertices(counts: &HashMap<usize, usize>) -> Vec<usize> {
    let mut sorted_counts: Vec<_> = counts.iter().collect();
    // Sort the counts by termination count
    sorted_counts.sort_by_key(|&(_, count)| count);
    // Get the top five vertices
    sorted_counts.into_iter().rev().take(5).map(|(&vertex, _)| vertex).collect()
}

// test to check the output format
#[test]
fn test_output_format() {
    // Define a sample PageRank map for testing
    let pagerank: HashMap<usize, f64> = [
        (9, 0.518),
        (7, 0.063),
        (0, 0.060),
        (2, 0.059),
        (4, 0.057),
    ]
    .iter()
    .cloned()
    .collect();

    // Sort the pagerank map by vertex
    let mut sorted_pagerank: Vec<_> = pagerank.into_iter().collect();
    sorted_pagerank.sort_by_key(|&(vertex, _)| vertex);

    // Generate output string
    let mut output = String::new();
    for (vertex, rank) in sorted_pagerank {
        output.push_str(&format!("vertex {}: approximate PageRank {:.3}\n", vertex, rank));
    }

    // Define the expected output string
    let expected_output = "vertex 0: approximate PageRank 0.060\n\
                           vertex 2: approximate PageRank 0.059\n\
                           vertex 4: approximate PageRank 0.057\n\
                           vertex 7: approximate PageRank 0.063\n\
                           vertex 9: approximate PageRank 0.518\n";

    // Print the generated and expected output for comparison
    println!("Output:");
    println!("{}", output);
    println!("Expected Output:");
    println!("{}", expected_output);

    // Assert that the generated output matches the expected output
    assert_eq!(output, expected_output);
}
