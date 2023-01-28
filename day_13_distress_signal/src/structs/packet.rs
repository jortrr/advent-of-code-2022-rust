use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PacketDataComparison {
    Continue,
    Ordered,
    Unordered,
}

#[derive(PartialEq, Clone)]
pub enum PacketData {
    List(Vec<PacketData>),
    Integer(u8),
}

impl PacketData {
    ///Parse a list string literal or slice into a PacketData::List, will return an error if the string cannot be parsed into a PacketData::List.
    pub fn parse_list(list_on_packet_line: &str) -> Result<PacketData, String> {
        PacketData::parse_list_print_recursion_level(list_on_packet_line, 0)
    }

    ///Print the current recursion level (4 spaces per recursion level) on the current terminal line.
    fn print_recursion_level(recursion_level: usize) {
        for _ in 0..recursion_level {
            print!("    ");
        }
    }

    ///Apend the integer_string as a PacketData::Integer to the PacketData if self == PacketData::List, or append nothing if value_string is empty
    fn append_value(&mut self, value_string: &mut String) {
        if !value_string.is_empty() {
            if let PacketData::List(ref mut l) = self {
                let integer: u8 = value_string.parse().expect(&format!(
                    "The string ({}) cannot be converted to a u8.",
                    value_string
                ));
                let integer: PacketData = PacketData::Integer(integer);
                //Add the Integer to our main list
                l.push(integer);
                *value_string = String::new();
            }
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
        let mut list_level: u16 = 0; //The current list level, gets incremented when '[' is encountered, decremented when ']' is encountered
        let mut found_list_begin_index = 0;
        let mut list = PacketData::List(Vec::new()); //Our return value

        for (i, c) in list_on_packet_line.chars().enumerate() {
            PacketData::print_recursion_level(recursion_level + 1);
            println!("(i: {}, c: {})", i, c);
            if c == '[' {
                list_level += 1;
            } else if c == ']' {
                list_level -= 1;
            }

            if list_level == 0 && c == ']' {
                list.append_value(&mut parsed_integer);
                return Ok(list);
            } else if list_level == 1 {
                //We're looking for a ']' to recursively call parse_list() on the substring containing the sub-list
                if c == ']' {
                    let found_list_last_index = i;
                    let sub_list = PacketData::parse_list_print_recursion_level(
                        &list_on_packet_line[found_list_begin_index..found_list_last_index + 1],
                        recursion_level + 1,
                    )?;
                    //Add the sub-list to our main list
                    if let PacketData::List(ref mut l) = list {
                        l.push(sub_list);
                    }
                } else if c == ',' || c == ']' {
                    list.append_value(&mut parsed_integer);
                    if c == ']' && list_level == 0 {
                        return Ok(list);
                    }
                } else if c != '[' {
                    parsed_integer.push(c);
                }
            } else if list_level == 2 && c == '[' {
                found_list_begin_index = i;
            }
        }

        Err(format!(
            "No PacketData::List was found in the string ({})",
            list_on_packet_line
        ))
    }

    ///Compares the left PacketData to the right PacketData recursily, returns whether the PacketData's are in the right order
    pub fn compare_print_recursion_level(
        left: &PacketData,
        right: &PacketData,
        recursion_level: usize,
    ) -> PacketDataComparison {
        match left {
            PacketData::List(l) => match right {
                PacketData::List(r) => {
                    //- Compare [l] vs [r]
                    PacketData::print_recursion_level(recursion_level);
                    println!("- Compare {:?} vs {:?}", left, right);
                    let mut l_iter = l.iter();
                    let mut r_iter = r.iter();
                    loop {
                        let l = l_iter.next();
                        let r = r_iter.next();
                        match l {
                            Some(left_data) => match r {
                                Some(right_data) => {
                                    //- Compare left_data vs right_data
                                    let comparison = PacketData::compare_print_recursion_level(
                                        left_data,
                                        right_data,
                                        recursion_level + 1,
                                    );
                                    match comparison {
                                        PacketDataComparison::Continue => continue,
                                        _ => return comparison,
                                    }
                                }
                                None => {
                                    //If the right list runs out of items first, the inputs are not in the right order.
                                    return PacketDataComparison::Unordered;
                                }
                            },
                            None => match r {
                                Some(_) => {
                                    //If the left list runs out of items first, the inputs are in the right order.
                                    return PacketDataComparison::Ordered;
                                }
                                None => {
                                    //If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
                                    match recursion_level {
                                        0 => return PacketDataComparison::Ordered,
                                        _ => return PacketDataComparison::Continue,
                                    }
                                }
                            },
                        }
                    }
                }

                PacketData::Integer(r) => {
                    //- Mixed types; convert right to [right] and retry comparison
                    PacketData::print_recursion_level(recursion_level);
                    println!(
                        "- Mixed types; convert {} to [{}] and retry comparison",
                        r, r
                    );
                    return PacketData::compare_print_recursion_level(
                        left,
                        &PacketData::List(vec![right.clone()]),
                        recursion_level + 1,
                    );
                }
            },

            PacketData::Integer(l) => match right {
                PacketData::List(r) => {
                    //- Mixed types; convert left to [left] and retry comparison
                    PacketData::print_recursion_level(recursion_level);
                    println!(
                        "- Mixed types; convert {} to [{}] and retry comparison",
                        l, l
                    );
                    return PacketData::compare_print_recursion_level(
                        &PacketData::List(vec![left.clone()]),
                        right,
                        recursion_level + 1,
                    );
                }

                PacketData::Integer(r) => {
                    //- Compare l vs r
                    PacketData::print_recursion_level(recursion_level);
                    println!("- Compare {} vs {}", l, r);
                    if l < r {
                        return PacketDataComparison::Ordered;
                    }
                    if l > r {
                        return PacketDataComparison::Unordered;
                    }
                    return PacketDataComparison::Continue;
                }
            },
        }
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

///https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#[cfg(test)]
mod tests {
    use crate::structs::packet::PacketDataComparison;

    use super::PacketData;

    fn test_left_and_right_list_strings(
        l_list: &str,
        r_list: &str,
        expected: PacketDataComparison,
    ) {
        let l_packet_data = PacketData::parse_list(l_list).unwrap();
        let r_packet_data = PacketData::parse_list(r_list).unwrap();
        let comparison =
            PacketData::compare_print_recursion_level(&l_packet_data, &r_packet_data, 0);
        assert_eq!(expected, comparison);
    }

    #[test]
    fn test_compare_print_recursion_level_1() {
        test_left_and_right_list_strings(
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            PacketDataComparison::Ordered,
        );
    }
    #[test]
    fn test_compare_print_recursion_level_2() {
        test_left_and_right_list_strings("[[1],[2,3,4]]", "[[1],4]", PacketDataComparison::Ordered);
    }
    #[test]
    fn test_compare_print_recursion_level_3() {
        test_left_and_right_list_strings("[9]", "[[8,7,6]]", PacketDataComparison::Unordered);
    }
    #[test]
    fn test_compare_print_recursion_level_4() {
        test_left_and_right_list_strings(
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            PacketDataComparison::Ordered,
        );
    }
    #[test]
    fn test_compare_print_recursion_level_5() {
        test_left_and_right_list_strings("[7,7,7,7]", "[7,7,7]", PacketDataComparison::Unordered);
    }
    #[test]
    fn test_compare_print_recursion_level_6() {
        test_left_and_right_list_strings("[]", "[3]", PacketDataComparison::Ordered);
    }
    #[test]
    fn test_compare_print_recursion_level_7() {
        test_left_and_right_list_strings("[[[]]]", "[[]]", PacketDataComparison::Unordered);
    }
    #[test]
    fn test_compare_print_recursion_level_8() {
        test_left_and_right_list_strings(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            PacketDataComparison::Unordered,
        );
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
