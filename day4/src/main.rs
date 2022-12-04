use std::fs;

struct Elf {
    from: u32,
    to: u32,
}

impl From<&str> for Elf {
    fn from(inp: &str) -> Self {
        let mut parts = inp.split('-').map(|x| x.parse().unwrap());
        let f = parts.next().unwrap();
        let t = parts.next().unwrap();
        Elf { from: f, to: t }
    }
}

impl Elf {
    fn contains(&self, other: &Elf) -> bool {
        (self.from <= other.from && self.to >= other.to)
            || (other.from <= self.from && other.to >= self.to)
    }
    fn overlaps(&self, other: &Elf) -> bool {
        (self.from <= other.from && self.to >= other.from)
            || (other.from <= self.from && other.to >= self.from)
    }
}

fn parse_elves(line: &str) -> (Elf, Elf) {
    let mut elves = line.split(',').map(Elf::from);
    let e1 = elves.next().unwrap();
    let e2 = elves.next().unwrap();
    (e1, e2)
}

fn count(input: &str, predicte: fn((Elf, Elf)) -> bool) -> usize {
    input
        .lines()
        .map(parse_elves)
        .map(predicte)
        .filter(|x| *x)
        .count()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let containing = count(&input, |(x, y)| x.contains(&y) || y.contains(&x));
    let overlapping = count(&input, |(x, y)| x.overlaps(&y));

    println!("Part 1: {}", containing);
    println!("Part 2: {}", overlapping);
}
