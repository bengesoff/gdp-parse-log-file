#[macro_use]
extern crate nom;

use std::fs;

mod parser;

fn main() {
    // Load the file into a &str
    let file_contents = fs::read_to_string("../Testbed_Log.log").unwrap();
    // Run the nom parser over the string
    let transmissions = parser::get_transmissions(&file_contents);
    // Count the total number of attempts
    let attempts = transmissions.iter().fold(0, |acc, t| acc + t.attempts.len());
    println!("Got {} transmissions. There were {} attempts which means that the average number of attempts per transmission was {}.", transmissions.len(), attempts, attempts as f64/transmissions.len() as f64);
}
