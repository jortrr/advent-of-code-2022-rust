use super::packet::Packet;

#[derive(Debug)]
pub struct PacketPair {
    left: Packet,
    right: Packet,
    ordered: bool,
}

impl PacketPair {
    pub fn new(left: Packet, right: Packet) -> PacketPair {
        PacketPair {
            left,
            right,
            ordered: false,
        }
    }
    ///Compare the order of the left and right Packets, store whether the PacketPair is ordered in self.ordered
    pub fn compare_packets(&mut self) {
        //TODO
    }
}
