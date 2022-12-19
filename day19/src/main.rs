use std::{collections::HashMap, fs};

type Robot = [usize; 3];
type NumBots = [usize; 4];
type Material = [usize; 3];

fn can_afford(cost: Robot, material: Material) -> bool {
    cost.iter().zip(material.iter()).all(|(c, m)| c <= m)
}
fn can_afford_two(cost: Robot, material: Material) -> bool {
    cost.iter().zip(material.iter()).all(|(c, m)| 2 * c <= *m)
}
fn buy_robot(cost: Robot, mut material: Material) -> Material {
    for (i, c) in cost.iter().enumerate() {
        material[i] -= c;
    }
    material
}

fn add(x: Material, y: &[usize]) -> Material {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

fn evaluate_blueprint(
    robots: &[Robot; 4],
    num_robots: NumBots,
    material: Material,
    time_left: usize,
    cache: &mut HashMap<(NumBots, Material, usize), usize>,
) -> usize {
    if time_left == 1 {
        return num_robots[3];
    }
    if let Some(&c) = cache.get(&(num_robots, material, time_left)) {
        return c;
    }

    let mut geodes = 0;
    if !can_afford(robots[3], material) {
        geodes = evaluate_blueprint(
            robots,
            num_robots,
            add(material, &num_robots[..3]),
            time_left - 1,
            cache,
        );
    }
    for i in 0..4 {
        if i != 3 && can_afford(robots[3], material) {
            continue;
        }
        if !can_afford(robots[i], material) {
            continue;
        }
        if can_afford_two(robots[i], material) {
            continue;
        }
        let new_mat = buy_robot(robots[i], material);
        let mut new_num = num_robots;
        new_num[i] += 1;

        geodes = geodes.max(evaluate_blueprint(
            robots,
            new_num,
            add(new_mat, &num_robots[..3]),
            time_left - 1,
            cache,
        ));
    }
    geodes += num_robots[3];
    cache.insert((num_robots, material, time_left), geodes);

    geodes
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let mut blueprints: Vec<[Robot; 4]> = vec![];
    for line in input.lines() {
        let mut robot_costs = line.split(". ");

        let mut cost = robot_costs
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>());
        let ore_cost = cost.next().unwrap();
        let ore_bot = [ore_cost, 0, 0];

        let mut cost = robot_costs
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>());
        let ore_cost = cost.next().unwrap();
        let clay_bot = [ore_cost, 0, 0];

        let mut cost = robot_costs
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>());
        let ore_cost = cost.next().unwrap();
        let clay_cost = cost.next().unwrap();
        let obsidian_bot = [ore_cost, clay_cost, 0];

        let mut cost = robot_costs
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>());
        let ore_cost = cost.next().unwrap();
        let obsidian_cost = cost.next().unwrap();
        let geode_bot = [ore_cost, 0, obsidian_cost];

        blueprints.push([ore_bot, clay_bot, obsidian_bot, geode_bot]);
    }
    let tot_quality = blueprints.iter().enumerate().fold(0, |acc, (i, bp)| {
        acc + (i + 1) * evaluate_blueprint(bp, [1, 0, 0, 0], [0, 0, 0], 24, &mut HashMap::new())
    });
    dbg!(tot_quality);
    let tot_geodes = blueprints.iter().take(3).fold(1, |acc, bp| {
        acc * evaluate_blueprint(bp, [1, 0, 0, 0], [0, 0, 0], 32, &mut HashMap::new())
    });
    dbg!(tot_geodes);
}
