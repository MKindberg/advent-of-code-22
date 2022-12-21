use std::{collections::HashMap, fs};

fn eval_monkey<'a>(
    monkey: &'a str,
    num_monkeys: & mut HashMap<&'a str, i64>,
    op_monkeys: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) {
    if num_monkeys.contains_key(monkey) {
        return;
    }

    if !num_monkeys.contains_key(op_monkeys[monkey].0) {
        eval_monkey(op_monkeys[monkey].0, num_monkeys, op_monkeys);
    }
    if !num_monkeys.contains_key(op_monkeys[monkey].2) {
        eval_monkey(op_monkeys[monkey].2, num_monkeys, op_monkeys);
    }

    let m1 = op_monkeys[monkey].0;
    let m2 = op_monkeys[monkey].2;
    num_monkeys.insert(
        monkey,
        match op_monkeys[monkey].1 {
            "+" => num_monkeys[m1] + num_monkeys[m2],
            "-" => num_monkeys[m1] - num_monkeys[m2],
            "*" => num_monkeys[m1] * num_monkeys[m2],
            "/" => num_monkeys[m1] / num_monkeys[m2],
            _ => panic!("Invalid operation"),
        },
    );
}

fn contains_humn<'a>(
    monkey: &'a str,
    op_monkeys: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) -> bool {
    if monkey == "humn" {
        return true;
    }
    if !op_monkeys.contains_key(monkey) {
        return false;
    }
    let m1 = op_monkeys[monkey].0;
    let m2 = op_monkeys[monkey].2;

    contains_humn(m1, op_monkeys) || contains_humn(m2, op_monkeys)
}

fn eval_eq<'a>(
    eq: i64,
    monkey: &'a str,
    num_monkeys: &mut HashMap<&'a str, i64>,
    op_monkeys: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) {
    if monkey == "humn" {
        num_monkeys.insert(monkey, eq);
        return;
    }
    if num_monkeys.contains_key(monkey) {
        return;
    }

    let m1 = op_monkeys[monkey].0;
    let m2 = op_monkeys[monkey].2;

    let mut mh = m1;
    let val = if !contains_humn(m1, op_monkeys) {
        eval_monkey(m1, num_monkeys, op_monkeys);
        mh = m2;

        match op_monkeys[monkey].1 {
            "+" => eq - num_monkeys[m1],
            "-" => num_monkeys[m1] - eq,
            "*" => eq / num_monkeys[m1],
            "/" => num_monkeys[m1] / eq,
            _ => panic!("Invalid operation"),
        }
    } else {
        eval_monkey(m2, num_monkeys, op_monkeys);
        match op_monkeys[monkey].1 {
            "+" => eq - num_monkeys[m2],
            "-" => eq + num_monkeys[m2],
            "*" => eq / num_monkeys[m2],
            "/" => eq * num_monkeys[m2],
            _ => panic!("Invalid operation"),
        }
    };

    eval_eq(val, mh, num_monkeys, op_monkeys);
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut num_monkeys: HashMap<&str, i64> = HashMap::new();
    let mut op_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    for line in input.lines() {
        let (monkey, data) = line.split_once(": ").unwrap();
        if let Ok(n) = data.parse::<i64>() {
            num_monkeys.insert(monkey, n);
        } else {
            let mut ops = data.split_whitespace();
            op_monkeys.insert(
                monkey,
                (
                    ops.next().unwrap(),
                    ops.next().unwrap(),
                    ops.next().unwrap(),
                ),
            );
        }
    }

    let mut nm = num_monkeys.clone();
    let mut om = op_monkeys.clone();
    eval_monkey("root", &mut nm, &mut om);
    println!("Part 1: {}", nm["root"]);

    let m1 = op_monkeys["root"].0;
    let m2 = op_monkeys["root"].2;
    eval_eq(nm[m2], m1, &mut num_monkeys, &mut op_monkeys);
    println!("Part 2: {}", num_monkeys["humn"]);
}
