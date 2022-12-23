use std::fs;

type Elf = (i32, i32);

fn add(elf: &Elf, c: (i32, i32)) -> Elf {
    (elf.0 + c.0, elf.1 + c.1)
}

fn alone(elf: &Elf, elves: &[Elf]) -> bool {
    for x in -1..=1 {
        for y in -1..=1 {
            if (x, y) != (0, 0) && elves.iter().any(|&e| e == add(elf, (x, y))) {
                return false;
            }
        }
    }
    true
}

fn other_elf(elf: &Elf, elves: &[Elf], directions: &[(i32, i32); 3]) -> bool {
    let want = directions.map(|d| add(elf, d));
    elves.iter().any(|e| want.contains(e))
}

// fn print(elves: &[Elf]) {
//     let max_x = elves.iter().map(|e| e.0).max().unwrap();
//     let max_y = elves.iter().map(|e| e.1).max().unwrap();
//     let min_x = elves.iter().map(|e| e.0).min().unwrap();
//     let min_y = elves.iter().map(|e| e.1).min().unwrap();
//
//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             if elves.iter().any(|e| e.0 == x && e.1 == y) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn empty_area(elves: &[Elf]) -> i32 {
    let max_x = elves.iter().map(|e| e.0).max().unwrap() + 1;
    let max_y = elves.iter().map(|e| e.1).max().unwrap() + 1;
    let min_x = elves.iter().map(|e| e.0).min().unwrap();
    let min_y = elves.iter().map(|e| e.1).min().unwrap();

    (max_x - min_x) * (max_y - min_y) - elves.len() as i32
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut elves: Vec<Elf> = vec![];
    for (y, l) in input.lines().enumerate() {
        for (x, b) in l.bytes().enumerate() {
            if b == b'#' {
                elves.push((x as i32, y as i32));
            }
        }
    }
    let mut want = elves.clone();
    let directions = [
        [-1, 0, 1].map(|x| (x, -1)),
        [-1, 0, 1].map(|x| (x, 1)),
        [-1, 0, 1].map(|y| (-1, y)),
        [-1, 0, 1].map(|y| (1, y)),
    ];

    for t in 0.. {
        if t == 10 {
            dbg!(empty_area(&elves));
        }
        let mut all_alone = true;
        assert!(want.len() == elves.len());
        for (w, e) in want.iter_mut().zip(&elves) {
            if alone(e, &elves) {
                continue;
            }
            all_alone = false;
            for i in 0..4 {
                let d = directions[(i + (t % 4)) % 4];
                if !other_elf(e, &elves, &d) {
                    *w = add(e, d[1]);
                    break;
                }
            }
        }

        if all_alone {
            dbg!(t+1);
            break;
        }
        for (e, w) in elves.iter_mut().zip(&want) {
            if e != w && want.iter().filter(|&x| x == w).count() == 1 {
                *e = *w;
            }
        }
        assert!(t < 1000);
    }
}
