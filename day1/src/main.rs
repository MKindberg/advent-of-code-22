use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let mut elves : Vec<u32> = vec![];
    let mut sum = 0;
    for l in input.lines() {
        if l.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += l.parse::<u32>().unwrap();
        }
    }

    elves.sort();
    println!("{}", elves.iter().rev().next().unwrap());
    println!("{}", elves.iter().rev().take(3).sum::<u32>());
}
