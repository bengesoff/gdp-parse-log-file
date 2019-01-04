#[macro_use]
extern crate nom;

mod parser;

fn main() {
    let header = parser::parse_transmission_header("Transmission ID: 45, Sent, Time(secs): 123453453\n--0");
    println!("{:?}",header);
}
