use std::fs;

fn turn(dir: u8, current: (i32, i32)) -> (i32, i32) {
    if dir == b'R' {
        match current {
            (0, 1) => (1, 0),
            (0, -1) => (-1, 0),
            (1, 0) => (0, -1),
            (-1, 0) => (0, 1),
            _ => panic!("Invalid direction"),
        }
    } else {
        match current {
            (0, 1) => (-1, 0),
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (-1, 0) => (0, -1),
            _ => panic!("Invalid direction"),
        }
    }
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
fn to_usize(a: (i32, i32)) -> (usize, usize) {
    (a.0.try_into().unwrap(), a.1.try_into().unwrap())
}

fn walk(map: &Vec<Vec<u8>>, mut current: (i32, i32), dir: (i32, i32), steps: i32) -> (i32, i32) {
    for _ in 0..steps {
        let mut n = add(current, dir);
        match dir {
            (0, 1) => {
                let c = to_usize(n);
                if c.1 >= map[c.0].len() || map[c.0][c.1] == b' ' {
                    for i in (0..c.1).rev() {
                        if i == 0 || map[c.0][i - 1] == b' ' {
                            n.1 = i as i32;
                            break;
                        }
                    }
                }
            }
            (0, -1) => {
                if n.1 < 0 || map[to_usize(n).0][to_usize(n).1] == b' ' {
                    n.1 += 1;
                    let c = to_usize(n);
                    for i in c.1 + 1..map[c.0].len() {
                        if i == map[c.0].len() - 1 || map[c.0][i + 1] == b' ' {
                            n.1 = i as i32;
                            break;
                        }
                    }
                }
            }
            (1, 0) => {
                let c = to_usize(n);
                if c.0 >= map.len() || map[c.0][c.1] == b' ' {
                    for i in (0..c.0).rev() {
                        if i == 0 || map[i - 1][c.1] == b' ' {
                            n.0 = i as i32;
                            break;
                        }
                    }
                }
            }
            (-1, 0) => {
                if n.0 < 0
                    || map[to_usize(n).0].len() <= n.1 as usize
                    || map[to_usize(n).0][to_usize(n).1] == b' '
                {
                    n.0 += 1;
                    let c = to_usize(n);
                    for i in c.0 + 1..map.len() {
                        if i == map.len() - 1 || map[i + 1][c.1] == b' ' {
                            n.0 = i as i32;
                            break;
                        }
                    }
                }
            }
            _ => panic!("Invalid direction"),
        };
        let c = to_usize(n);
        if map[c.0][c.1] == b'#' {
            break;
        }
        current = n;
    }
    current
}

fn dir_score(dir: (i32, i32)) -> i32 {
    match dir {
        (0, 1) => 0,
        (0, -1) => 2,
        (1, 0) => 1,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let (m, i) = input.split_once("\n\n").unwrap();

    let width = m.lines().next().unwrap().len();
    let map: Vec<Vec<u8>> = m
        .lines()
        .map(|l| {
            l.bytes()
                .chain(" ".repeat(width - l.len()).bytes())
                .collect()
        })
        .collect();

    let directions: Vec<u8> = i.bytes().filter(|c| c.is_ascii_uppercase()).collect();
    let steps: Vec<i32> = i
        .split_terminator(&['R', 'L'][..])
        .flat_map(|s| s.parse::<i32>())
        .chain(
            i.rsplit_terminator(&['R', 'L'][..])
                .flat_map(|s| s.parse::<i32>())
                .take(1),
        )
        .collect();

    let mut dir = (0, 1);
    let mut pos = (0, map[0].iter().position(|&c| c != b' ').unwrap() as i32);

    for (i, &s) in steps.iter().enumerate() {
        pos = walk(&map, pos, dir, s);
        let p = to_usize(pos);
        if pos == (0, 0) || map[p.0][p.1] == b' ' {
            break;
        }
        if let Some(&d) = directions.get(i) {
            dir = turn(d, dir);
        }
    }
    println!(
        "Part 1: {}",
        (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + dir_score(dir)
    );
}
