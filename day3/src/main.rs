use std::fs;

fn get_score(letter: &u8) -> u32 {
    (if letter.is_ascii_lowercase() {
        1 + letter - b'a'
    } else {
        27 + letter - b'A'
    }) as u32
}
fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let mut score: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();

    for l in &lines {
        let mid = l.len() / 2;
        score += get_score(
            &l.bytes()
                .take(mid)
                .find(|c| l.bytes().skip(mid).any(|x| x == *c))
                .unwrap(),
        );
    }

    println!("Part 1: {}", score);
    score = 0;
    for i in (0..lines.len()).step_by(3) {
        score += get_score(
            &lines[i]
                .bytes()
                .find(|&c| lines[i + 1].contains(c as char) && lines[i + 2].contains(c as char))
                .unwrap(),
        );
    }
    println!("Part 2: {}", score);
}
