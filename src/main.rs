#[macro_use]
extern crate nom;

use std::fs;
use std::env;

mod parser;

fn main() {
    // Get log file location from arguments
    let filename = env::args().nth(1).expect("No filename supplied");
    // Load the file into a &str
    let file_contents = fs::read_to_string(filename).unwrap();
    // Run the nom parser over the string
    let transmissions = parser::get_transmissions(&file_contents);
    // Count the total number of attempts
    let attempts = transmissions.iter().fold(0, |acc, t| acc + t.attempts.len());

    // Count the number of undetected errors
    let undetected_errors = transmissions.iter().filter(|t| {
        let last_attempt = t.attempts.iter().last().unwrap();
        return last_attempt.error && !last_attempt.detected_error;
    }).count();

    println!("Got {} transmissions. There were {} attempts which means that the average number of attempts per transmission was {}.", transmissions.len(), attempts, attempts as f64/transmissions.len() as f64);
    println!("There were also {} undetected errors, where the codec did not detect an error but the comparison engine did.", undetected_errors);
    println!("Therefore the residual BER was {}%.", 100.0 * undetected_errors as f64 / attempts as f64);
}
