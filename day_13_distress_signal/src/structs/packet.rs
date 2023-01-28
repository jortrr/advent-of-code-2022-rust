use crate::enums::packet_data::{PacketData, PacketDataComparison};

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

    pub fn compare(left: &Packet, right: &Packet) -> bool {
        let comparison: PacketDataComparison =
            PacketData::compare_print_recursion_level(&left.packet_data, &right.packet_data, 0);
        match comparison {
            PacketDataComparison::Ordered => true,
            PacketDataComparison::Unordered => false,
            PacketDataComparison::Continue => panic!("PacketData::compare_print_recursion_level(&left.packet_data, &right.packet_data, 0) == PacketDataComparison::Continue, this should never happen.")
        }
    }
}
