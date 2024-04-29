//Q3 
use std::io;

fn main() {
    println!("Enter a non-negative integer:"); //asking user to enter an integer

    let mut input = String::new(); // Create a mutable string to store user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim any leading or trailing whitespace (cleaning the int)
    let k: u32 = input.parse().expect("Not a good number!");

    let mut sum: u128 = 0; // Initialize a variable

    for i in 1..=k { // Iterate from 1 up to and including the input number
        sum += i as u128 * i as u128; // Add the square of the current number to the sum
    }

    println!("The sum of squares up to {} is: {}", k, sum);
}

