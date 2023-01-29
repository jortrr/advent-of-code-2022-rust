use super::{packet::Packet, packet_pair::PacketPair};

pub struct Signal {
    packet_pairs: Vec<PacketPair>,
}

impl Signal {
    ///Return the sum of the indices of the correctly ordered pairs (the answer to AoC puzzle part one)
    pub fn compute_sum_of_ordered_pair_indices(&self) -> usize {
        let mut sum: usize = 0;
        for (i, packet_pair) in self.packet_pairs.iter().enumerate() {
            if packet_pair.ordered() {
                sum += i + 1;
            }
        }
        sum
    }

    ///Returns the decoder key (the indices of the divider packets [[2]] and [[6]] multiplied together) (the answer to AoC puzzle part two).
    ///Will copy all Packets in `self.packet_pairs` and the divider packets into a Vector, and sort the Vector using the Merge Sort Algorithm first.
    pub fn compute_decoder_key(&self) -> usize {
        //Copy all Packets in self.packet_pairs and the divider packets into a Vector, and sort the Vector using the Merge Sort Algorithm
        let first_divider_packet: Packet = Packet::from("[[2]]");
        let second_divider_packet: Packet = Packet::from("[[6]]");
        let mut signal_packets: Vec<Packet> =
            vec![first_divider_packet.clone(), second_divider_packet.clone()];
        for packet_pair in &self.packet_pairs {
            signal_packets.push(packet_pair.left().clone());
            signal_packets.push(packet_pair.right().clone());
        }
        Packet::merge_sort(&mut signal_packets, 0);

        //Compute the decoder key
        let mut decoder_key: usize = 1;
        for (i, packet) in signal_packets.iter().enumerate() {
            if *packet == first_divider_packet || *packet == second_divider_packet {
                decoder_key *= i + 1;
            }
        }
        decoder_key
    }

    pub fn add_packet_pair(&mut self, packet_pair: PacketPair) {
        self.packet_pairs.push(packet_pair);
    }

    ///Identify for all `PacketPairs` in `packet_pairs` whether they are currently in the right order
    pub fn compare_order_of_packet_pairs(&mut self) {
        for packet_pair in &mut self.packet_pairs {
            packet_pair.compare_packets();
        }
    }

    ///Print all the `PacketPairs` in `self.packet_pairs` to the terminal
    pub fn print(&self) {
        println!("=== Signal ===");
        for (i, packet_pair) in self.packet_pairs.iter().enumerate() {
            println!("== Pair {} ==", i + 1);
            println!("{:?}", packet_pair);
            println!();
        }
    }

    pub fn new() -> Signal {
        Signal {
            packet_pairs: Vec::new(),
        }
    }
}
