use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    // nop maps to 0 and addX N maps to [0, N]
    let increments = input
        .lines()
        .flat_map(|l| l.split_whitespace().map(|w| w.parse::<i32>().unwrap_or(0)));

    let mut sum = 1;
    let values: Vec<i32> = increments
        .map(|x| {
            sum += x;
            sum
        })
        .collect();

    println!(
        "Part 1: {}",
        values
            .iter()
            .enumerate()
            .map(|(c, v)| (c + 2, v))
            .skip(18)
            .step_by(40)
            .take(6)
            .map(|(n, x)| { x * n as i32 })
            .sum::<i32>()
    );

    println!("Part 2:");
    print!("■");
    for (c, &v) in values.iter().enumerate().map(|(c, v)| (c + 1, v)) {
        let cycle = c as i32 % 40;
        if (v - 1..=v + 1).contains(&cycle) {
            print!("■");
        } else {
            print!(" ");
        }
        if c % 40 == 39 {
            println!();
        }
    }
}
