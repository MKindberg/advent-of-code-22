use std::{
    collections::HashSet,
    fs,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn mv_head(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Invalid direction"),
    }
}
fn mv_to(t: &mut Point, h: Point) {
    let Point(diff_x, diff_y) = h - *t;

    *t += if (diff_x, diff_y) != (0, 0) && diff_x.abs() + diff_y.abs() > 2 {
        (diff_x.signum(), diff_y.signum())
    } else if diff_x == 0 && diff_y.abs() > 1 {
        (0, diff_y.signum())
    } else if diff_y == 0 && diff_x.abs() > 1 {
        (diff_x.signum(), 0)
    } else {
        (0, 0)
    };
}

fn print(rope: &[Point]) {
    let mut grid: Vec<Vec<i32>> = vec![vec![-1; 10]; 10];
    for (i, r) in rope.iter().enumerate().rev() {
        grid[r.1 as usize][r.0 as usize] = i as i32;
    }
    for r in grid.iter().rev() {
        for c in r {
            if *c == -1 {
                print!(".");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn unique_tail_pos(input: &str, rope_len: usize) -> usize {
    let mut positions: HashSet<Point> = HashSet::new();
    let mut rope: Vec<Point> = vec![Point(0, 0); rope_len];
    positions.insert(rope[0]);

    for (dir, steps) in input.lines().flat_map(|x| x.split_once(' ')) {
        for _ in 0..steps.parse().unwrap() {
            rope[0] += mv_head(dir);
            let mut prev = rope[0];
            for current in rope.iter_mut().skip(1) {
                mv_to(current, prev);
                prev = *current;
            }
            positions.insert(*rope.last().unwrap());
        }
    }
    positions.len()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    println!("Part 1: {}", unique_tail_pos(&input, 2));
    println!("Part 2: {}", unique_tail_pos(&input, 10));
}
