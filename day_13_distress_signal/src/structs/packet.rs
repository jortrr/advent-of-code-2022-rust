use crate::enums::packet_data::{PacketData, PacketDataComparison};

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    packet_data: PacketData,
}

impl Packet {
    pub fn from(packet_line: &str) -> Packet {
        Packet {
            packet_data: PacketData::parse_list(packet_line).unwrap(),
        }
    }

    pub fn compare(left: &Packet, right: &Packet, recursion_level: usize) -> bool {
        let comparison: PacketDataComparison = PacketData::compare_print_recursion_level(
            &left.packet_data,
            &right.packet_data,
            recursion_level,
        );
        match comparison {
            PacketDataComparison::Ordered => true,
            PacketDataComparison::Unordered => false,
            PacketDataComparison::Continue => panic!("PacketData::compare_print_recursion_level(&left.packet_data, &right.packet_data, 0) == PacketDataComparison::Continue, this should never happen.")
        }
    }

    ///Print the current recursion level (4 spaces per recursion level) on the current terminal line.
    fn print_recursion_level(recursion_level: usize) {
        for _ in 0..recursion_level {
            print!("    ");
        }
    }

    fn print_packets(packets: &[Packet]) {
        for (i, packet) in packets.iter().enumerate() {
            print!("{:?}", packet.packet_data);
            if i + 1 < packets.len() {
                print!(", ");
            }
        }
    }

    ///Sort the Packet slice recursively using the Merge Sort Algorithm, until packets is in the right order. See: <https://www.geeksforgeeks.org/merge-sort/>
    pub fn merge_sort(packets: &mut [Packet], recursion_level: usize) {
        Packet::print_recursion_level(recursion_level);
        print!("- merge_sort(");
        Packet::print_packets(packets);
        println!(")");

        if packets.len() <= 1 {
            //Already sorted
        } else if packets.len() == 2 {
            //Sort the 2 packets
            let ordered: bool = Packet::compare(&packets[0], &packets[1], recursion_level + 1);
            Packet::print_recursion_level(recursion_level + 1);
            if ordered {
                print!("- ordered: ");
                Packet::print_packets(packets);
                println!();
                //Already sorted
            } else {
                print!("- unordered: ");
                Packet::print_packets(packets);
                println!();
                let first_packet_copy: Packet = packets[0].clone();
                packets[0] = packets[1].clone();
                packets[1] = first_packet_copy;
                //Sorted
            }
        } else {
            let middle: usize = packets.len() / 2;
            Packet::merge_sort(&mut packets[..middle], recursion_level + 1);
            Packet::merge_sort(&mut packets[middle..], recursion_level + 1);
            //Sort the 2 packet slices
            Packet::print_recursion_level(recursion_level + 1);
            print!("- before merge: ");
            Packet::print_packets(packets);
            println!();
            let mut left_packet_slice_copies: Vec<Packet> = Vec::new();
            for packet in packets[..middle].iter() {
                left_packet_slice_copies.push(packet.clone());
            }
            let mut right_packet_slice_copies: Vec<Packet> = Vec::new();
            for packet in packets[middle..].iter() {
                right_packet_slice_copies.push(packet.clone());
            }
            let mut left_iter = left_packet_slice_copies.iter();
            let mut right_iter = right_packet_slice_copies.iter();
            let mut packets_iter = packets.iter_mut();
            let mut left = left_iter.next();
            let mut right = right_iter.next();
            loop {
                let packet: Option<&mut Packet> = packets_iter.next();
                if packet.is_none() {
                    break;
                }
                let packet: &mut Packet = packet.unwrap();
                match (left, right) {
                    (Some(l), Some(r)) => {
                        //Compare l vs r
                        let ordered: bool = Packet::compare(l, r, recursion_level + 1);
                        if ordered {
                            *packet = l.clone();
                            left = left_iter.next();
                        } else {
                            *packet = r.clone();
                            right = right_iter.next();
                        }
                    }
                    (Some(l), None) => {
                        //Append l to packets
                        *packet = l.clone();
                        left = left_iter.next();
                    }
                    (None, Some(r)) => {
                        //Append r to packets
                        *packet = r.clone();
                        right = right_iter.next();
                    }
                    (None, None) => {
                        //Done
                        break;
                    }
                }
            }
            //Sorted
        }
        Packet::print_recursion_level(recursion_level + 1);
        print!("- merged: ");
        Packet::print_packets(packets);
        println!();
    }
}

///<https://doc.rust-lang.org/book/ch11-01-writing-tests.html>
#[cfg(test)]
mod tests {
    use super::Packet;

    fn create_packet_vector_from_packet_lines(packet_lines: Vec<&str>) -> Vec<Packet> {
        let mut packets: Vec<Packet> = Vec::new();
        for packet_line in packet_lines {
            packets.push(Packet::from(packet_line));
        }
        packets
    }

    #[test]
    fn test_merge_sort() {
        let packet_lines: Vec<&str> = vec![
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "[9]",
            "[[8,7,6]]",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "[7,7,7,7]",
            "[7,7,7]",
            "[]",
            "[3]",
            "[[[]]]",
            "[[]]",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "[[2]]",
            "[[6]]",
        ];
        let mut packets: Vec<Packet> = create_packet_vector_from_packet_lines(packet_lines);

        Packet::merge_sort(&mut packets[..], 0);

        let sorted_packets_lines: Vec<&str> = vec![
            "[]",
            "[[]]",
            "[[[]]]",
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "[[1],[2,3,4]]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[[1],4]",
            "[[2]]",
            "[3]",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "[[6]]",
            "[7,7,7]",
            "[7,7,7,7]",
            "[[8,7,6]]",
            "[9]",
        ];
        let sorted_packets: Vec<Packet> =
            create_packet_vector_from_packet_lines(sorted_packets_lines);

        assert_eq!(sorted_packets.len(), packets.len());
        for i in 0..packets.len() {
            assert_eq!(sorted_packets[i], packets[i], "(i: {})", i);
        }
    }
}
