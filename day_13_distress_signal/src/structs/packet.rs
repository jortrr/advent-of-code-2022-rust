#[derive(PartialEq, Debug)]
pub enum PacketData {
    List(Vec<PacketData>),
    Integer(u8),
}

impl PacketData {
    pub fn parse_list(list_on_packet_line: &str) -> Result<PacketData, String> {
        println!("parse_list({})", list_on_packet_line);
        if list_on_packet_line.chars().nth(0).unwrap() != '[' {
            return Err(format!("The PacketData::List string ({}) is invalid, because the first character is not a '['.", list_on_packet_line));
        }
        if list_on_packet_line.chars().nth_back(0).unwrap() != ']' {
            return Err(format!("The PacketData::List string ({}) is invalid, because the last character is not a ']'.", list_on_packet_line));
        }
        let mut parsed_integer: String = String::new();
        let mut found_a_list: bool = false;
        let mut found_list_begin_index = 0;
        let mut found_list_last_index = 0;
        let mut list = PacketData::List(Vec::new()); //Our return value
        for (i, c) in list_on_packet_line[1..].chars().enumerate() {
            println!("  (i: {}, c: {})", i, c);
            if c == '[' {
                found_a_list = true;
                found_list_begin_index = i + 1;
            }
            if found_a_list {
                //We're looking for a ']' to recursively call parse_list() on the substring containing the sub-list
                if c == ']' {
                    found_list_last_index = i + 1;
                    found_a_list = false;
                    let sub_list = PacketData::parse_list(
                        &list_on_packet_line[found_list_begin_index..found_list_last_index + 1],
                    )?;
                    //Add the sub-list to our main list
                    if let PacketData::List(ref mut l) = list {
                        l.push(sub_list);
                    }
                }
            } else {
                //We're looking for Integers, separated by either a comma, or ended by a ']'
                if c == ',' || c == ']' {
                    if parsed_integer.is_empty() {
                        continue;
                    }
                    let integer: u8 = parsed_integer.parse().unwrap();
                    let integer: PacketData = PacketData::Integer(integer);
                    //Add the Integer to our main list
                    if let PacketData::List(ref mut l) = list {
                        l.push(integer);
                    }
                    parsed_integer = String::new();
                    if c == ']' {
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

pub struct Packet {
    packet_data: PacketData,
}

impl Packet {
    pub fn from(packet_line: &str) -> Packet {
        let mut packet_data: PacketData = PacketData::List(Vec::new());
        for c in packet_line.chars() {
            if c == '[' {
                packet_data = PacketData::List(Vec::new());
            } else if c == ']' {
            }
        }
        Packet {
            packet_data: PacketData::List(Vec::new()),
        } //TODO
    }
}
