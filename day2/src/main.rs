use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let mut score1 = 0;
    let mut score2 = 0;
    let points = HashMap::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
    for l in input.lines() {
        let c: Vec<i32> = l.split_whitespace().map(|x| points[x]).collect();

        score1 += (c[1] + 1)
        + match (c[1] - c[0] + 3) % 3  {
            0 => 3,
            1 => 6,
            _ => 0
        };

        score2 += match c[1] {
            0 => ((c[0] + 2) % 3) + 1,
            2 => ((c[0] + 1) % 3) + 1 + 6,
            _ => c[0] + 1 + 3,
        };
    }

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
}
