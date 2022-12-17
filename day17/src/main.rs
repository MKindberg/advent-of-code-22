use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

struct Rock {
    parts: Vec<(usize, usize)>,
}

impl Rock {
    fn new(turn: usize, height: usize) -> Self {
        let h = height + 3;
        match turn % 5 {
            0 => Rock {
                parts: vec![(2, h), (3, h), (4, h), (5, h)],
            },
            1 => Rock {
                parts: vec![(2, h + 1), (3, h), (3, h + 1), (3, h + 2), (4, h + 1)],
            },
            2 => Rock {
                parts: vec![(2, h), (3, h), (4, h), (4, h + 1), (4, h + 2)],
            },
            3 => Rock {
                parts: vec![(2, h), (2, h + 1), (2, h + 2), (2, h + 3)],
            },
            4 => Rock {
                parts: vec![(2, h), (2, h + 1), (3, h), (3, h + 1)],
            },
            _ => panic!("Not possible"),
        }
    }

    fn fall(&mut self, tunnel: &HashSet<(usize, usize)>, wind: &u8) -> bool {
        match wind {
            b'<' => {
                if self.can_move_left(tunnel) {
                    for i in 0..self.parts.len() {
                        self.parts[i].0 -= 1;
                    }
                }
            }
            b'>' => {
                if self.can_move_right(tunnel) {
                    for i in 0..self.parts.len() {
                        self.parts[i].0 += 1;
                    }
                }
            }
            _ => panic!("Invalid wind direction"),
        };
        let cont = self.can_move_down(tunnel);
        if cont {
            for i in 0..self.parts.len() {
                self.parts[i].1 -= 1;
            }
        }
        cont
    }
    fn can_move_left(&self, tunnel: &HashSet<(usize, usize)>) -> bool {
        for r in &self.parts {
            if tunnel.contains(&(r.0 - 1, r.1)) || r.0 == 0 {
                return false;
            }
        }
        true
    }
    fn can_move_right(&self, tunnel: &HashSet<(usize, usize)>) -> bool {
        for r in &self.parts {
            if tunnel.contains(&(r.0 + 1, r.1)) || r.0 == 6 {
                return false;
            }
        }
        true
    }
    fn can_move_down(&self, tunnel: &HashSet<(usize, usize)>) -> bool {
        for r in &self.parts {
            if tunnel.contains(&(r.0, r.1 - 1)) || r.1 == 1 {
                return false;
            }
        }
        true
    }
}

#[allow(dead_code)]
fn print(tunnel: &HashSet<(i32, i32)>) {
    for y in (0..15).rev() {
        for x in 0..=6 {
            if tunnel.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

type StoneType = usize; // The type of stone
type LeftEdge = usize; // How far from the left side the stone is
type Depth = [usize; 7]; // The depth of each column
type CacheKey = (StoneType, LeftEdge, Depth);
type Height = usize; // The current height of the stone tower
type StoneNr = usize; // The number of stones in the pile
type CycleIdx = usize; // The first cycle seems to be a false one
type CacheVal = (Height, StoneNr, CycleIdx);
fn calc_height(winds: &[u8], stones: usize) -> usize {
    let mut tunnel: HashSet<(usize, usize)> = HashSet::new();
    let mut height = 1;
    let mut wind_idx = 0;
    let mut cont = true;
    let mut cache: HashMap<CacheKey, CacheVal> = HashMap::new();
    let mut skipped_height = 0;

    let mut s = 0;
    while s < stones {
        let mut rock = Rock::new(s, height);
        while cont {
            cont = rock.fall(&tunnel, &winds[wind_idx]);
            wind_idx += 1;
            if wind_idx % winds.len() == 0 {
                wind_idx = 0;

                let depth = tunnel.iter().fold([0; 7], |mut acc, item| {
                    acc[item.0] = acc[item.0].min(height - item.1);
                    acc
                });

                let left_edge = rock.parts.iter().map(|x| x.0).min().unwrap();

                let key = (s % 5, left_edge, depth);
                if let Some((_, _, 0)) = cache.get(&key) {
                    cache.insert(key, (height, s, 1));
                } else if let Some((cached_height, st, 1)) = cache.get(&key) {
                    let period = s - st;
                    let periods_to_skip = (stones - st) / period - 1;
                    s += period * periods_to_skip;
                    skipped_height += (height - cached_height) * periods_to_skip;
                } else {
                    cache.insert(key, (height, s, 0));
                };
            }
        }
        cont = true;
        for r in rock.parts {
            tunnel.insert((r.0, r.1));
        }
        height = *tunnel.iter().map(|(_, y)| y).max().unwrap() + 1;
        s += 1;
    }
    height - 1 + skipped_height
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let wind_dir = input.trim().as_bytes();

    let timer = Instant::now();
    let height = calc_height(wind_dir, 2022);
    let time = timer.elapsed();
    println!("Part 1: {} in {}ms", height, time.as_millis());

    let timer = Instant::now();
    let height = calc_height(wind_dir, 1_000_000_000_000);
    let time = timer.elapsed();
    println!("Part 2: {} in {}ms", height, time.as_millis());
}
