use std::fmt;

#[derive(PartialEq)]
pub enum PacketData {
    List(Vec<PacketData>),
    Integer(u8),
}

impl PacketData {
    ///Parse a list string literal or slice into a PacketData::List, will return an error if the string cannot be parsed into a PacketData::List.
    pub fn parse_list(list_on_packet_line: &str) -> Result<PacketData, String> {
        PacketData::parse_list_print_recursion_level(list_on_packet_line, 0)
    }

    //Print the current recursion level (4 spaces per recursion level) on the current terminal line.
    fn print_recursion_level(recursion_level: usize) {
        for _ in 0..recursion_level {
            print!("    ");
        }
    }

    ///Parse a list string literal or slice into a PacketData::List, will return an error if the string cannot be parsed into a PacketData::List. Will also print at the current recursion level to the terminal.
    fn parse_list_print_recursion_level(
        list_on_packet_line: &str,
        recursion_level: usize,
    ) -> Result<PacketData, String> {
        PacketData::print_recursion_level(recursion_level);
        println!("parse_list({})", list_on_packet_line);
        if list_on_packet_line.chars().nth(0).unwrap() != '[' {
            return Err(format!("The PacketData::List string ({}) is invalid, because the first character is not a '['.", list_on_packet_line));
        }
        if list_on_packet_line.chars().nth_back(0).unwrap() != ']' {
            return Err(format!("The PacketData::List string ({}) is invalid, because the last character is not a ']'.", list_on_packet_line));
        }
        let mut parsed_integer: String = String::new();
        let mut amount_of_sub_lists_encountered: u16 = 1;
        let mut found_a_list: bool = false;
        let mut found_list_begin_index = 0;
        let mut list = PacketData::List(Vec::new()); //Our return value
        for (i, c) in list_on_packet_line[1..].chars().enumerate() {
            PacketData::print_recursion_level(recursion_level);
            PacketData::print_recursion_level(1);
            println!("(i: {}, c: {})", i, c);
            if c == '[' {
                amount_of_sub_lists_encountered += 1;
                //println!("++");
            } else if c == ']' {
                amount_of_sub_lists_encountered -= 1;
                //println!("--");
            }
            if found_a_list {
                //We're looking for a ']' to recursively call parse_list() on the substring containing the sub-list
                if c == ']' && amount_of_sub_lists_encountered == 1 {
                    let found_list_last_index = i + 1;
                    found_a_list = false;
                    let sub_list = PacketData::parse_list_print_recursion_level(
                        &list_on_packet_line[found_list_begin_index..found_list_last_index + 1],
                        recursion_level + 1,
                    )?;
                    //Add the sub-list to our main list
                    if let PacketData::List(ref mut l) = list {
                        l.push(sub_list);
                    }
                }
            } else {
                //We're looking for Integers, separated by either a comma, or ended by a ']'
                if c == '[' {
                    found_a_list = true;
                    found_list_begin_index = i + 1;
                } else if c == ',' || c == ']' {
                    if !parsed_integer.is_empty() {
                        let integer: u8 = parsed_integer.parse().expect(&format!(
                            "The string ({}) cannot be converted to a u8.",
                            parsed_integer
                        ));
                        let integer: PacketData = PacketData::Integer(integer);
                        //Add the Integer to our main list
                        if let PacketData::List(ref mut l) = list {
                            l.push(integer);
                        }
                        parsed_integer = String::new();
                    }
                    if c == ']' && amount_of_sub_lists_encountered == 0 {
                        return Ok(list);
                    }
                } else {
                    parsed_integer.push(c);
                }
            }
        }
        Err(format!(
            "No PacketData::List was found in the string ({})",
            list_on_packet_line
        ))
    }
}

impl fmt::Debug for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketData::List(ref l) => {
                let mut debug_string: String = String::new();
                for (i, packet_data) in l.iter().enumerate() {
                    debug_string = format!("{}{:?}", debug_string, packet_data);
                    if i < l.len() - 1 {
                        debug_string.push(',');
                    }
                }
                write!(f, "[{}]", debug_string)
            }
            PacketData::Integer(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    packet_data: PacketData,
}

impl Packet {
    pub fn from(packet_line: &str) -> Packet {
        Packet {
            packet_data: PacketData::parse_list(packet_line).unwrap(),
        }
    }
}
