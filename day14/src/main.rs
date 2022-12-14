#![allow(clippy::needless_return)]
use std::{collections::HashSet, fs};

fn get_rocks(from: &str, to: &str) -> HashSet<(i32, i32)> {
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut f = from.split(',').flat_map(|x| x.parse::<i32>());
    let mut from_x = f.next().unwrap();
    let mut from_y = f.next().unwrap();

    let mut t = to.split(',').flat_map(|x| x.parse::<i32>());
    let to_x = t.next().unwrap();
    let to_y = t.next().unwrap();

    let diff_x = to_x - from_x;
    let diff_y = to_y - from_y;

    rocks.insert((from_x, from_y));
    while (from_x, from_y) != (to_x, to_y) {
        from_x += diff_x.signum();
        from_y += diff_y.signum();
        rocks.insert((from_x, from_y));
    }
    return rocks;
}

fn drop_sand(rocks: &mut HashSet<(i32, i32)>, bot: i32, stop_at_bot: bool) -> usize {
    let mut count = 0;
    'a: loop {
        let mut sand = (500, 0);
        loop {
            if !rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 += -1;
                sand.1 += 1;
            } else if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                rocks.insert(sand);
                count += 1;
                if sand == (500, 0) {
                    break 'a;
                }
                break;
            }
            if sand.1 == bot {
                if stop_at_bot {
                    break 'a;
                } else {
                    rocks.insert(sand);
                    count += 1;
                    break;
                }
            }
        }
    }
    return count;
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        for (from, to) in line.split(" -> ").zip(line.split(" -> ").skip(1)) {
            for rock in get_rocks(from, to) {
                rocks.insert(rock);
            }
        }
    }
    let bot = rocks.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    let part1 = drop_sand(&mut rocks, bot, true);
    println!("Part 1: {}", part1);

    let part2 = part1 + drop_sand(&mut rocks, bot + 1, false);
    println!("Part 2: {}", part2);
}
