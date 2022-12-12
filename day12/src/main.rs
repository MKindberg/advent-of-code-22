use std::fs;

macro_rules! eval_step {
    ($terrain:ident, $steps:ident, $current:ident, $y_to:expr, $x_to:expr, $down:ident) => {
        let from = $terrain[$current.0][$current.1];
        let from_s = $steps[$current.0][$current.1];
        let to = $terrain[$y_to][$x_to];
        let to_s = $steps[$y_to][$x_to];
        if (($down && from + 1 >= to) || (!$down && from - 1 <= to)) && from_s + 1 < to_s {
            $steps[$y_to][$x_to] = from_s + 1;
            step($terrain, $steps, &($y_to, $x_to), $down);
        }
    };
}

fn step(terrain: &Vec<Vec<u8>>, steps: &mut Vec<Vec<usize>>, current: &(usize, usize), down: bool) {
    if current.1 > 0 {
        eval_step!(terrain, steps, current, current.0, current.1 - 1, down);
    }
    if current.1 < terrain[0].len() - 1 {
        eval_step!(terrain, steps, current, current.0, current.1 + 1, down);
    }
    if current.0 > 0 {
        eval_step!(terrain, steps, current, current.0 - 1, current.1, down);
    }
    if current.0 < terrain.len() - 1 {
        eval_step!(terrain, steps, current, current.0 + 1, current.1, down);
    }
}

fn print(steps: &Vec<Vec<usize>>) {
    for l in steps {
        for s in l {
            print!("{} ", s);
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let mut terrain: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut steps: Vec<Vec<usize>> = terrain
        .iter()
        .map(|l| l.iter().map(|_| usize::MAX).collect())
        .collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..terrain.len() {
        for j in 0..terrain[0].len() {
            if terrain[i][j] == b'S' {
                start = (i, j);
                steps[i][j] = 0;
                terrain[i][j] = b'a';
            }
            if terrain[i][j] == b'E' {
                end = (i, j);
                terrain[i][j] = b'z';
            }
        }
    }

    step(&terrain, &mut steps, &start, true);
    println!("Part 1: {}", steps[end.0][end.1]);

    let mut steps: Vec<Vec<usize>> = terrain
        .iter()
        .map(|l| l.iter().map(|_| usize::MAX).collect())
        .collect();
    steps[end.0][end.1] = 0;

    steps = terrain
        .iter()
        .map(|l| l.iter().map(|_| usize::MAX).collect())
        .collect();
    steps[end.0][end.1] = 0;
    step(&terrain, &mut steps, &(end.0, end.1), false);
    let mut min = usize::MAX;
    for i in 0..steps.len() {
        for j in 0..steps[0].len() {
            if terrain[i][j] == b'a' && steps[i][j] < min {
                min = steps[i][j];
            }
        }
    }

    println!("Part 2: {}", min);
}
