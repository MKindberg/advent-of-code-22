use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let cmds = input[2..].split("\n$ ");

    let mut path: Vec<&str> = vec![];
    let mut sizes: HashMap<String, u128> = HashMap::new();
    for cmd in cmds {
        match &cmd[0..2] {
            "cd" => {
                if &cmd[3..] == ".." {
                    path.pop();
                } else {
                    path.push(&cmd[3..]);
                    sizes.insert(path.join("/"), 0);
                }
            }
            "ls" => {
                let dir_size: u128 = cmd
                    .lines()
                    .filter(|x| x.as_bytes()[0].is_ascii_digit())
                    .flat_map(|x| x.split_whitespace().flat_map(|n| n.parse::<u128>()))
                    .sum();
                for i in 0..path.len() {
                    let p = path[..=i].join("/");
                    sizes.insert(p.clone(), sizes[&p] + dir_size);
                }
            }
            _ => println!("Invalid command {}", cmd),
        }
    }
    let sum_sub_100k: u128 = sizes.values().filter(|&x| *x <= 100000).sum();
    println!("Part 1: {}", sum_sub_100k);

    let min_delete = 30000000 - (70000000 - sizes[&"/".to_string()]);
    let delete = sizes.values().filter(|&x| *x >= min_delete).min().unwrap();
    println!("Part 2: {}", delete);
}
