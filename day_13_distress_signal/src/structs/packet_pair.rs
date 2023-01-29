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
    ///Compare the order of the left and right Packets, store whether the `PacketPair` is ordered in self.ordered
    pub fn compare_packets(&mut self) {
        self.ordered = Packet::compare(&self.left, &self.right, 0);
    }

    pub fn ordered(&self) -> bool {
        self.ordered
    }

    pub fn left(&self) -> &Packet {
        &self.left
    }

    pub fn right(&self) -> &Packet {
        &self.right
    }
}
