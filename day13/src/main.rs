use std::fs;

enum Packet {
    Packet(Vec<Packet>),
    Item(i32),
}

fn parse_packet(inp: &[u8]) -> Packet {
    let mut num_str = "".to_string();
    let mut items: Vec<Packet> = vec![];
    let mut i = 0;
    while i < inp.len() {
        match inp[i] {
            b'[' => items.push(parse_packet(&inp[i + 1..])),
            b']' => {
                if !num_str.is_empty() {
                    return (i, Packet::Item(num_str.parse().unwrap()));
                } else {
                    return (i, Packet::Packet(items));
                }
            }
            b',' => return Packet,
            b'_' => num_str += b,
        }
    }
    return Packet::Item(0);
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let pa = input.split("\n\n");

    let mut packets: Vec<Packet> = vec![];
    let packets = pa.map(|x| x.lines().map(|l| l.as_bytes()).map(parse_packet));
}
