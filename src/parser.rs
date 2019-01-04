use nom::*;

#[derive(Debug,PartialEq)]
pub enum Direction {
    Sent,
    Received,
}

#[derive(Debug,PartialEq)]
pub struct Transmission {
    pub header: Header,
    pub attempts: Vec<Attempt>,
}

#[derive(Debug,PartialEq)]
pub struct Header {
    pub id: u32,
    pub direction: Direction,
}

#[derive(Debug,PartialEq)]
pub struct Attempt {
    pub attempt_number: u32,
    pub corrupted: u32,
    pub uncorrupted: u32,
    pub error: bool,
    pub detected_error: bool,
}

/// Parse the input string and get a vector of Transmission structs back
pub fn get_transmissions(file: &str) -> Vec<Transmission> {
    let res = many_transmissions(file);
    match res {
        Ok((_, all_transmissions)) => all_transmissions,
        Err(e) => panic!("parse error: {:?}", e)
    }
}

named!(transmission_header<&str, Header>,
   do_parse!(
       tag!("Transmission ID: ") >>
       id: map_res!(digit1, from_int) >>
       comma_space >>
       direction: map_res!(alt!(tag!("Sent") | tag!("Received")), get_direction) >>
       comma_space >>
       many1!(not_line_ending) >>
       line_ending >> 
       (Header {id, direction})
   )
);

named!(transmission_attempt<&str, Attempt>,
    do_parse!(
        tag!("--") >>
        attempt_number: hex_packet >>
        comma >>
        corrupted: hex_packet >>
        comma >>
        uncorrupted: hex_packet >>
        comma >>
        error: map_res!(digit, to_bool) >>
        comma >>
        detected_error: map_res!(digit, to_bool) >>
        opt!(line_ending) >>
        (Attempt {
            attempt_number,
            corrupted,
            uncorrupted,
            error,
            detected_error
        })
    )
);

named!(transmission<&str, Transmission>,
    do_parse!(
        header: transmission_header >>
        attempts: many0!(transmission_attempt) >>
        (Transmission {
            header,
            attempts
        })
    )
);

named!(many_transmissions<&str, Vec<Transmission>>,
   many1!(transmission)
);

named!(hex_packet<&str, u32>,
    map_res!(take_while_m_n!(1,6,is_hex_digit), from_hex)
);

named!(comma_space<&str, &str>, tag!(", "));
named!(comma<&str, &str>, tag!(","));

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn from_int(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn to_bool(input: &str) -> Result<bool, u32> {
    match input {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(0)
    }
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

#[test]
fn parse_attempt() {
    assert_eq!(transmission_attempt("--0,17a883,17a983,0,0\n"), Ok(("", Attempt {
        attempt_number: 0,
        corrupted: 0x17a883,
        uncorrupted: 0x17a983,
        error: false,
        detected_error: false
    })));
}

#[test]
fn parse_transmission() {
    let test = "Transmission ID: 45, Sent, Time(secs): 123453453\n--0,17a883,17a983,0,0\n--0,17a883,17a983,0,0\n\n\n";
    assert_eq!(transmission(test), Ok(("\n\n", Transmission {
        header: Header {
            id: 45,
            direction: Direction::Sent,
        },
        attempts: vec![Attempt {
            attempt_number: 0,
            corrupted: 0x17a883,
            uncorrupted: 0x17a983,
            error: false,
            detected_error: false
        }, Attempt {
            attempt_number: 0,
            corrupted: 0x17a883,
            uncorrupted: 0x17a983,
            error: false,
            detected_error: false
        }]
    })));
}

