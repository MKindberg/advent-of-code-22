use std::{collections::HashSet, fs};

type Pos = (i32, i32);
#[derive(Copy, Clone)]
struct Storm {
    pos: Pos,
    dir: u8,
    width: i32,
    height: i32,
}

impl Storm {
    fn mv(&mut self) {
        match self.dir {
            b'>' => {
                if self.pos.0 == self.width - 1 {
                    self.pos.0 = 1
                } else {
                    self.pos.0 += 1
                }
            }
            b'<' => {
                if self.pos.0 == 1 {
                    self.pos.0 = self.width - 1
                } else {
                    self.pos.0 -= 1
                }
            }
            b'v' => {
                if self.pos.1 == self.height - 1 {
                    self.pos.1 = 1
                } else {
                    self.pos.1 += 1
                }
            }
            b'^' => {
                if self.pos.1 == 1 {
                    self.pos.1 = self.height - 1
                } else {
                    self.pos.1 -= 1
                }
            }
            _ => panic!("Invalid direction"),
        };
    }
}

fn print(storms: &[Storm]) {
    let (w, h) = (storms[0].width, storms[0].height);

    for y in 1..h {
        for x in 1..w {
            let mut c: u8 = b'.';
            for s in storms {
                if s.pos == (x, y) {
                    c = s.dir;
                }
            }
            print!("{}", c as char);
        }
        println!();
    }
    println!();
}

fn add(pos: Pos, inc: Pos) -> Pos {
    (pos.0 + inc.0, pos.1 + inc.1)
}

fn storm_in(storms: &[Storm], pos: Pos) -> bool {
    for s in storms {
        if s.pos == pos {
            return true;
        }
    }
    false
}

fn step(storms: &[Storm], positions: HashSet<Pos>, width: i32, height: i32) -> HashSet<Pos> {
    let mut new_positions: HashSet<Pos> = HashSet::new();

    for p in positions {
        for d in [(1, 0), (0, 1), (0, -1), (-1, 0), (0, 0)] {
            let next = add(p, d);
            let (x, y) = next;
            if (x, y) != (1, 0) && (x, y) != (width - 1, height) {
                if (y <= 0) || (x <= 0) || (y >= height) || (x >= width) {
                    continue;
                }
            }
            if !storm_in(storms, next) {
                new_positions.insert(next);
            }
        }
    }
    new_positions
}

fn find_exit(storms: &mut [Storm], start: Pos, end: Pos) -> i32 {
    let mut positions: HashSet<Pos> = HashSet::new();
    positions.insert(start);
    let (width, height) = (storms[0].width, storms[0].height);
    for i in 1.. {
        for s in storms.iter_mut() {
            s.mv();
        }
        positions = step(storms, positions, width, height);
        if positions.contains(&end) {
            for s in storms.iter_mut() {
                s.mv();
            }
            return i + 1;
        }
    }
    unreachable!();
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let width = input.bytes().position(|b| b == b'\n').unwrap() as i32 - 1;
    let height = input.lines().count() as i32 - 1;
    let mut storms: Vec<Storm> = vec![];
    for (y, l) in input.lines().enumerate() {
        for (x, b) in l.bytes().enumerate() {
            if ![b'.', b'#'].contains(&b) {
                storms.push(Storm {
                    pos: (x as i32, y as i32),
                    dir: b,
                    width,
                    height,
                });
            }
        }
    }
    let trip1 = find_exit(&mut storms, (1, 0), (width - 1, height - 1));
    println!("Part 1: {}", trip1);
    let trip2 = find_exit(&mut storms, (width - 1, height), (1, 1));
    let trip3 = find_exit(&mut storms, (1, 0), (width - 1, height - 1));
    println!("Part 2: {}", trip1 + trip2 + trip3);
}
