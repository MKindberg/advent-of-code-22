use std::fs;

fn mv(nums: &mut Vec<(usize, i64)>, elem: usize) {
    let pos = nums.iter().position(|x| x.0 == elem).unwrap();
    let e = nums.remove(pos);
    let  new_pos = (pos as i64 + e.1).rem_euclid(nums.len() as i64);
    nums.insert(new_pos as usize, e);
}

fn get_coordinates(nums: Vec<(usize, i64)>) -> i64 {
    let zero = nums.iter().position(|x| x.1 == 0).unwrap();
    let n1 = nums[(zero + 1000) % nums.len()].1;
    let n2 = nums[(zero + 2000) % nums.len()].1;
    let n3 = nums[(zero + 3000) % nums.len()].1;

    n1 + n2 + n3
}
fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut nums: Vec<(usize, i64)> = input
        .lines()
        .flat_map(|x| x.parse::<i64>())
        .enumerate()
        .collect();
    for i in 0..nums.len() {
        mv(&mut nums, i);
    }
    println!("Part 1: {}", get_coordinates(nums));

    let mut nums: Vec<(usize, i64)> = input
        .lines()
        .flat_map(|x| x.parse::<i64>())
        .map(|x| x * 811589153)
        .enumerate()
        .collect();
    for _ in 0..10 {
        for i in 0..nums.len() {
            mv(&mut nums, i);
        }
    }
    println!("Part 2: {}", get_coordinates(nums));
}
