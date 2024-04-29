use std::fs::File; // Importing the File struct, prelude module, Rng trait, and  BufReader
use std::io::prelude::*; 
use rand::Rng; 
use std::io::{BufReader, BufRead}; 

// Generate a file with n random numbers from -1,000,000 to 1,000,000
// and a randomly generated class of 0 or 1
fn generate_file(path: &str, n: usize) {
    let mut file = File::create(path).expect("Unable to create file"); // Attempt to create a new file at the given path.

    // Generating n random numbers and associated classes.
    for _ in 0..n {
        let rng = rand::thread_rng().gen_range(-1_000_000..=1_000_000) as i32; // Generate a random number within the specified range.
        let rand_class = rng.abs() % 2; // Generate a random class (0 or 1) based on the absolute value of the random number.
        let class = rand_class.to_string(); // Convert to a string.
        let s: String = format!("{},{}\n", rng, class); // Format 
        file.write_all(s.as_bytes()).expect("Unable to write file"); // Write the formatted string to the file.
    }
}

// Read a file and return a vector of tuples
// each tuple contains a random number and a randomly assigned class
fn read_file(path: &str) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new(); // Initialize an empty vector to store tuples of random numbers and classes.
    let file = File::open(path).expect("Could not open file"); // Attempt to open the file at the given path.
    let buf_reader = BufReader::new(file); // Create a buffered reader for the file.

    // Iterate over each line in the file
    for line in buf_reader.lines() {
        let line_str = line.expect("Error reading"); // Extract the line as a string
        let mut iter = line_str.trim().split(','); // Split the string by comma
        let x = iter.next().unwrap().parse::<i32>().unwrap(); // Parse the first part of the string as a number.
        let y = iter.next().unwrap().parse::<i32>().unwrap(); // Parse the second part of the string as a class.
        result.push((x, y)); // Push the tuple of number and class into the result vector.
    }
    result 
}

// Predict the class based on the split point and class direction
fn predict(split_point: i32, data: &Vec<(i32, i32)>, class_left: i32) -> f64 {
    let mut correct = 0.0; 
    // Iterate over each tuple 
    for &(number, actual_class) in data.iter() {
        // Determine the predicted class 
        let predicted_class = if (class_left == 0 && number <= split_point) || (class_left == 1 && number > split_point) {0} else {1};
        if actual_class == predicted_class { // Check if the predicted class matches the actual class.
            correct += 1.0; // Increment the count of correct predictions.
        }    
    }
    correct / data.len() as f64 // Calculate and return the accuracy as a ratio of correct predictions to the total number of data points.
}

// Find the best split point and class direction for the given data
fn best_split(data: &Vec<(i32, i32)>) -> (i32, i32, f64) {
    let mut best_split = 0; // Initialize the best split point.
    let mut best_accuracy = 0.0; // Initialize the best accuracy.
    let mut best_class_left = 0; // Initialize the best class direction.

    // Iterate over each possible class direction (0 and 1).
    for class_left in 0..=1 {
        // Iterate over each split point in the data.
        for &(split, _) in data.iter() {
            let accuracy = predict(split, &data, class_left); // Calculate the accuracy for the current split point and class direction.
            if accuracy > best_accuracy { // Check if the accuracy is better than the best accuracy found so far.
                best_accuracy = accuracy; // Update the best accuracy.
                best_split = split; // Update the best split point.
                best_class_left = class_left; // Update the best class direction.
            }
        }
    }
    (best_split, best_class_left, best_accuracy) // Return the best split point, class direction, and accuracy.
}

fn main() {
    generate_file("test_data.txt", 500); // Generate a file with 500 random numbers and classes.
    let data = read_file("test_data.txt"); // Read the generated file and store the data.
    let best_partition = best_split(&data); // Find the best partition for the data.
    
    // Print the results 
    println!("The best partition for the data is to split at {:?} with any x <= {:?} being classified as class {:?}.", best_partition.0, best_partition.0, best_partition.1 );
    println!("Any x <= the partition will be classified as {:?}. Any x > the partition will be classified as {:?}.", best_partition.1, if best_partition.1 == 0 {1} else {0});
    println!("The accuracy of this partition is {:?}.", best_partition.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict() {
        let data = vec![(-2, 1), (-1,0), (0,0), (1,1), (2,1), (3,1), (4,0), (5,0), (6,0), (7,1)];
        assert_eq!(predict(0, &data, 0), 0.6); // Test the predict function will retrn the accuracy of the given partition and direction of the class     

    }

    #[test]
    fn test_best_split() {
        let data = vec![(-2, 0), (-1,0), (0,0), (1,1), (2,1), (3,1), (4,0), (5,0), (6,0), (7,1)];
        assert_eq!(best_split(&data), (0, 0, 0.7)); // Test the best_split function with a known dataset. 
    }

    #[test]
    fn test_predict_one_class_left() {
        let data = vec![(-5, 1), (-4,1), (-3,1), (-2,1), (-1,1), (0,0), (1,0), (2,0), (3,0), (4,0), (5,0)];
        assert_eq!(predict(0, &data, 1), 0.9090909090909091); // Test the predict function  with one class on the left 
    }

    #[test]
    fn test_off_center_left() {
        let data = vec![(-5, 1), (-4,1), (-3,0), (-2,0), (-1,0), (0,0), (1,0), (2,0), (3,0), (4,0), (5,0)];
        assert_eq!(best_split(&data), (-4, 1, 1.0)); // Test the best_split function with dataset that is off center twards negative numbers 
    }
}



