use std::{collections::HashSet, fs, time::Instant};

type Cube = [i32; 3];

fn get_neighbors(cubes: &HashSet<Cube>, cube: Cube) -> Vec<Cube> {
    get_surrounding(cube)
        .into_iter()
        .filter(|c| cubes.contains(c))
        .collect()
}

fn count_sides(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|c| 6 - get_neighbors(cubes, *c).len())
        .sum()
}
fn get_surrounding(cube: Cube) -> Vec<Cube> {
    let mut surrounding: Vec<Cube> = vec![];
    for i in 0..3 {
        for d in [-1, 1] {
            let mut c = cube;
            c[i] += d;
            surrounding.push(c);
        }
    }
    surrounding
}

fn count_outsides(cubes: &HashSet<Cube>) -> usize {
    let max = cubes.iter().fold([0, 0, 0], |acc, cube| {
        [
            acc[0].max(cube[0] + 1),
            acc[1].max(cube[1] + 1),
            acc[2].max(cube[2] + 1),
        ]
    });
    let min = cubes.iter().fold([0, 0, 0], |acc, cube| {
        [
            acc[0].min(cube[0] - 1),
            acc[1].min(cube[1] - 1),
            acc[2].min(cube[2] - 1),
        ]
    });

    let mut to_check: Vec<Cube> = vec![min];
    let mut checked: HashSet<Cube> = HashSet::new();

    let mut count = 0;
    while let Some(c) = to_check.pop() {
        let neighbors = get_neighbors(cubes, c);
        let surrounding = get_surrounding(c);
        count += neighbors.len();
        for s in surrounding
            .into_iter()
            .filter(|c| {
                c.iter().zip(min).all(|(x, y)| x >= &y) && c.iter().zip(max).all(|(x, y)| x <= &y)
            })
            .filter(|c| !neighbors.contains(c))
            .filter(|c| !checked.contains(c))
        {
            if !to_check.contains(&s) {
                to_check.push(s);
            }
        }
        checked.insert(c);
    }
    count
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let cubes: HashSet<Cube> = input
        .lines()
        .map(|l| l.split(',').flat_map(|w| w.parse::<i32>()))
        .map(|mut c| [c.next().unwrap(), c.next().unwrap(), c.next().unwrap()])
        .collect();

    let timer = Instant::now();
    let count = count_sides(&cubes);
    let time = timer.elapsed();
    println!("Part 1: {} in {}µs", count, time.as_micros());

    let timer = Instant::now();
    let count = count_outsides(&cubes);
    let time = timer.elapsed();
    println!("Part 2: {} in {}ms", count, time.as_millis());
}
