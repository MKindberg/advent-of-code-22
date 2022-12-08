use std::{collections::HashSet, fs};

type Matrix<T> = Vec<Vec<T>>;

macro_rules! calc_score {
    ($rest_iter:expr, $tree:expr) => {
        $rest_iter.count()
            - $rest_iter
                .skip_while(|&&x| x < $tree)
                .skip(1)
                .count()
    };
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let trees: Matrix<u8> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect();
    let mut score: Matrix<usize> = trees
        .iter()
        .map(|l| l.iter().map(|_| 1).collect::<Vec<usize>>())
        .collect();
    let width = trees.len();
    let height = trees[0].len();

    let mut trees2: Matrix<u8> = Vec::new();
    for c in 0..width {
        trees2.push(Vec::new());
        for t in &trees {
            trees2[c].push(t[c]);
        }
    }

    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for (y, tt)  in trees.iter().take(height-1).enumerate().skip(1) {
        for (x, t)  in tt.iter().take(width-1).enumerate().skip(1) {
            let rest = &tt[..x];
            score[y][x] *= calc_score!(rest.iter().rev(), *t);
            if t > rest.iter().max().unwrap_or(&0) {
                visible.insert((y, x));
            }

            let rest = &trees[y][x + 1..];
            score[y][x] *= calc_score!(rest.iter(), *t);
            if t > tt[x + 1..].iter().max().unwrap_or(&0) {
                visible.insert((y, x));
            }
        }
    }

    for (y, tt)  in trees2.iter().take(height-1).enumerate().skip(1) {
        for (x, t)  in tt.iter().take(width-1).enumerate().skip(1) {
            let rest = &tt[..x];
            score[x][y] *= calc_score!(rest.iter().rev(), *t);

            if t > tt[..x].iter().max().unwrap_or(&0) {
                visible.insert((x, y));
            }
            let rest = &tt[x + 1..];
            score[x][y] *= calc_score!(rest.iter(), *t);

            if t > tt[x + 1..].iter().max().unwrap_or(&0) {
                visible.insert((x, y));
            }
        }
    }

    let num_visible = visible.len() + 2*trees.len() + 2*trees[0].len() - 4;
    let max_score = score.iter().flat_map(|x| x.iter().max()).max().unwrap();
    println!("Part 1: {}", num_visible);
    println!("Part 2: {}", max_score);
}
