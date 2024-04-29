// data_generator.rs

// Import necessary modules: File I/O operations, Buffered writing to file,  Random number generation
use std::fs::File;                      
use std::io::{BufWriter, Write};        
use rand::prelude::*;                   

// generate data file containing directed edges
pub fn generate_data_file(num_vertices: usize) {
    // Define the file path where the data is stored
    let file_path = "data.txt";
    
    // Create a new file and a buffered writer
    let mut file = BufWriter::new(File::create(file_path).expect("Failed to create file"));

    // Write the number of vertices to the file as the first line
    writeln!(file, "{}", num_vertices).expect("Failed to write to file");

    // Generate directed edges for each vertex
    let mut rng = rand::thread_rng();  // Initialize random number generator
    for vertex in 0..num_vertices {     // Iterate over each vertex
        // For each vertex, create directed edges to a fraction of other vertices
        for _ in 0..(num_vertices / 10) {
            // Generate a random target vertex within the range of vertices
            let target = rng.gen_range(0..num_vertices);
            // Write the directed edge (source, target) to the file
            writeln!(file, "{} {}", vertex, target).expect("Failed to write to file");
        }
    }
}

