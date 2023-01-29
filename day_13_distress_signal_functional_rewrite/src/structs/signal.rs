use super::packet::Packet;

pub struct Signal {
    packets: Vec<Packet>,
}

impl Signal {
    ///Return the sum of the indices of the correctly ordered pairs (the answer to AoC puzzle part one)
    pub fn compute_sum_of_ordered_pair_indices(&self) -> usize {
        let sum: usize = self
            .packets
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .filter(|(i, packet)| Packet::compare(packet, &self.packets[i + 1], 0))
            .map(|(i, _)| (i / 2) + 1)
            .sum();

        sum
    }

    ///Returns the decoder key (the indices of the divider packets [[2]] and [[6]] multiplied together) (the answer to AoC puzzle part two).
    ///Will copy all Packets in `self.packets` and the divider packets into a Vector, and sort the Vector using the Merge Sort Algorithm first.
    pub fn compute_decoder_key(&self) -> usize {
        //Copy all Packets in self.packets and the divider packets into a Vector, and sort the Vector using the Merge Sort Algorithm
        let first_divider_packet: Packet = Packet::from("[[2]]");
        let second_divider_packet: Packet = Packet::from("[[6]]");
        let mut signal_packets: Vec<Packet> =
            vec![first_divider_packet.clone(), second_divider_packet.clone()];
        signal_packets.append(&mut self.packets.clone());

        Packet::merge_sort(&mut signal_packets, 0);

        //Compute the decoder key
        let first_divider_package_index: usize = 1 + signal_packets
            .iter()
            .position(|p| *p == first_divider_packet)
            .unwrap();
        let second_divider_package_index: usize = 1 + signal_packets
            .iter()
            .position(|p| *p == second_divider_packet)
            .unwrap();

        let decoder_key: usize = first_divider_package_index * second_divider_package_index;

        decoder_key
    }

    ///Print all the `Packet`s in `self.packets` to the terminal
    pub fn print(&self) {
        println!("=== Signal ===");
        for (i, packet) in self.packets.iter().enumerate() {
            if i % 2 == 0 {
                println!("== Pair {} ==", i / 2 + 1);
            }
            println!("{:?}", packet);
            if i % 2 == 1 {
                println!();
            }
        }
    }

    pub fn new(packets: Vec<Packet>) -> Signal {
        Signal { packets }
    }
}
