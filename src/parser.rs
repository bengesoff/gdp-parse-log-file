use nom::*;

#[derive(Debug,PartialEq)]
pub enum Direction {
    Sent,
    Received,
}

#[derive(Debug,PartialEq)]
pub struct Header {
    pub id: u32,
    pub direction: Direction,
}

#[derive(Debug,PartialEq)]
pub struct Attempt {
    pub corrupted: &str,
    pub uncorrupted: &str,
    pub error: bool,
    pub detected_error: bool,
}

pub fn parse_transmission_header(line: &str) -> Header {
    match transmission_header(line) {
        Ok((_, result)) => result,
        Err(_) => panic!("Failed")
    }
}

named!(transmission_header<&str, Header>,
   do_parse!(
       tag!("Transmission ID: ") >>
       id: map_res!(digit1, from_int) >>
       comma >>
       direction: map_res!(alt!(tag!("Sent") | tag!("Received")), get_direction) >>
       comma >>
       many1!(not_line_ending) >>
       tag!("\n") >>
       (Header {id, direction})
   )
);

named!(comma<&str, &str>, tag!(", "));

fn from_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

fn get_direction(input: &str) -> Result<Direction, u32> {
    match input {
        "Sent" => Ok(Direction::Sent),
        "Received" => Ok(Direction::Received),
        _ => Err(0)
    }
}

#[test]
fn parse_header() {
    assert_eq!(transmission_header("Transmission ID: 45, Sent, Time(secs): 123453453\n--0"), Ok(("--0", Header {
        id: 45,
        direction: Direction::Sent,
    })));
}

