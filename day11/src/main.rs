use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: (u8, i64),
    test_div: i64,
    throw_true: usize,
    throw_false: usize,
    inspect_counter: usize,
    worry_div: i64,
    worry_mod: i64,
}

impl Monkey {
    fn eval_op(&self, item: i64) -> i64 {
        let op: i64 = if self.operation.1 == -1 {
            item
        } else {
            self.operation.1
        };
        match self.operation.0 {
            b'+' => item + op,
            b'-' => item - op,
            b'*' => item * op,
            b'/' => item / op,
            _ => panic!("Invalid operation"),
        }
    }

    fn throw_to(&mut self) -> Vec<(i64, usize)> {
        self.inspect_counter += self.items.len();
        let actions = self
            .items
            .iter()
            .map(|i| self.eval_op(*i) / self.worry_div % self.worry_mod)
            .map(|i| {
                (
                    i,
                    if i % self.test_div == 0 {
                        self.throw_true
                    } else {
                        self.throw_false
                    },
                )
            })
            .collect();
        self.items.clear();
        actions
    }
}

impl From<&str> for Monkey {
    fn from(monkey_data: &str) -> Self {
        let mut data = monkey_data.lines().skip(1);
        let items: Vec<i64> = data
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .flat_map(|x| x.parse())
            .collect();

        let ops = data
            .next()
            .unwrap()
            .split_once(" = old ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap();
        let operation = (ops.0.as_bytes()[0], ops.1.parse::<i64>().unwrap_or(-1));

        let test_div: i64 = data
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse())
            .next()
            .unwrap();

        let throw_true: usize = data
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse())
            .next()
            .unwrap();

        let throw_false: usize = data
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse())
            .next()
            .unwrap();

        Monkey {
            items,
            operation,
            test_div,
            throw_true,
            throw_false,
            inspect_counter: 0,
            worry_div: 3,
            worry_mod: 0,
        }
    }
}

fn play(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let actions = monkeys[m].throw_to();
            for (item, catcher) in actions {
                monkeys[catcher].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspect_counter);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspect_counter)
        .product::<usize>()
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let worry_mod: i64 = monkeys.iter().map(|m| m.test_div).product();
    monkeys.iter_mut().for_each(|x| x.worry_mod += worry_mod);

    println!("Part 1: {}", play(monkeys.clone(), 20));
    for m in &mut monkeys {
        m.worry_div = 1;
    }
    println!("Part 2: {}", play(monkeys.clone(), 10000));
}
