use std::fs;

fn mv1(state: &mut [Vec<u8>], from: usize, to: usize, num: usize) {
    for _ in 0..num {
        let elem = state[from].pop().unwrap();
        state[to].push(elem);
    }
}
fn mv2(state: &mut [Vec<u8>], from: usize, to: usize, num: usize) {
    let mut tmp: Vec<u8> = vec![];
    for _ in 0..num {
        let elem = state[from].pop().unwrap();
        tmp.push(elem);
    }
    state[to].append(&mut tmp);
}

fn lift(
    state: &mut [Vec<u8>],
    moves: &str,
    mover: fn(&mut [Vec<u8>], usize, usize, usize),
) -> String {
    for i in moves.lines() {
        let mut is = i
            .split(' ')
            .skip(1)
            .step_by(2)
            .flat_map(|x| x.parse::<usize>());
        let num = is.next().unwrap();
        let from = is.next().unwrap();
        let to = is.next().unwrap();
        mover(state, from - 1, to - 1, num);
    }
    state.iter().map(|x| *x.last().unwrap() as char).collect()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let (init, moves) = input.split_once("\n\n").unwrap();
    let mut init = init.lines().rev();
    let num_stacks = init
        .next()
        .unwrap()
        .trim_end()
        .bytes()
        .last()
        .unwrap()
        .wrapping_sub(b'0');

    let mut state: Vec<Vec<u8>> = vec![vec![]; num_stacks.into()];

    for l in init {
        l.as_bytes()[..]
            .chunks(4)
            .map(|x| x.get(1).unwrap())
            .enumerate()
            .for_each(|(i, &x)| {
                if x != b' ' {
                    state[i].push(x)
                }
            });
    }

    let s1 = lift(&mut state.clone(), moves, mv1);
    let s2 = lift(&mut state, moves, mv2);
    println!("{}", s1);
    println!("{}", s2);
}
