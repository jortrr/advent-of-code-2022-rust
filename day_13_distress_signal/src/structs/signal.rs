use super::packet_pair::PacketPair;

pub struct Signal {
    packet_pairs: Vec<PacketPair>,
}

impl Signal {
    ///Return the sum of the indices of the correctly ordered pairs (the answer to AoC puzzle part one)
    pub fn sum_of_ordered_pair_indices(&self) -> usize {
        //TODO
        0
    }

    pub fn add_packet_pair(&mut self, packet_pair: PacketPair) {
        self.packet_pairs.push(packet_pair);
    }

    ///Identify for all PacketPairs in packet_pairs whether they are currently in the right order
    pub fn compare_order_of_packet_pairs(&mut self) {
        //TODO
    }
}
