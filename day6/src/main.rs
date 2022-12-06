use std::{collections::HashSet, fs};

fn message_start(data: &str, l: usize) -> usize {
    data.as_bytes()
        .windows(l)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == w.len())
        .unwrap()
        + l
}
fn message_start2(data: &str, l: usize) -> usize {
    data.as_bytes()
        .windows(l)
        .position(|w| w.iter().map(|x| 1 << (x - b'a')).sum::<u32>().count_ones() as usize == l)
        .unwrap()
        + l
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    println!("Part 1: {}", message_start(&input, 4));
    println!("Part 1: {}", message_start2(&input, 4));
    println!("Part 2: {}", message_start(&input, 14));
    println!("Part 2: {}", message_start2(&input, 14));
}
