use super::packet_pair::PacketPair;

pub struct Signal {
    packet_pairs: Vec<PacketPair>,
}

impl Signal {
    ///Return the sum of the indices of the correctly ordered pairs (the answer to AoC puzzle part one)
    pub fn sum_of_ordered_pair_indices(&self) -> usize {
        let mut sum: usize = 0;
        for (i, packet_pair) in self.packet_pairs.iter().enumerate() {
            if packet_pair.ordered() {
                sum += i + 1;
            }
        }
        sum
    }

    pub fn add_packet_pair(&mut self, packet_pair: PacketPair) {
        self.packet_pairs.push(packet_pair);
    }

    ///Identify for all PacketPairs in packet_pairs whether they are currently in the right order
    pub fn compare_order_of_packet_pairs(&mut self) {
        for packet_pair in &mut self.packet_pairs {
            packet_pair.compare_packets();
        }
    }

    ///Print all the PacketPairs in self.packet_pairs to the terminal
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
