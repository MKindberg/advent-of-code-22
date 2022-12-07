use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|x| x.lines().flat_map(|l| l.parse::<u32>()).sum())
        .collect();

    elves.sort();
    println!("{}", elves.iter().rev().next().unwrap());
    println!("{}", elves.iter().rev().take(3).sum::<u32>());
}
