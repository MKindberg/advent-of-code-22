use std::fs;

fn snafu_to_dec(snafu: &str) -> i64 {
    let mut mul = 1;
    let mut dec = 0;
    for s in snafu.bytes().rev() {
        dec += match s {
            b'=' => -2 * mul,
            b'-' => -mul,
            b'0' => 0,
            b'1' => mul,
            b'2' => 2 * mul,
            _ => panic!("Invalid character"),
        };
        mul *= 5;
    }
    dec
}

fn f(mut n:i64) -> i64 {
    let mut s = 1;
    while n > 0 {
        s += n;
        n /= 5;
    }
    s
}
fn dec_to_snafu(mut num: i64) -> String {
    let mut snafu = "".to_string();

    let mut mul = 1;
    while num > 2 * mul {
        mul *= 5;
    }

    while mul > 1 {
        dbg!(num, mul);
        if (num - 2 * mul).abs() < 2 * f(mul / 5) {
            snafu += "2";
            num -= 2 * mul;
        } else if (num - mul).abs() < 2 * f(mul / 5) {
            snafu += "1";
            num -= mul;
        } else if (num + 2 * mul).abs() < 2 * f(mul / 5) {
            snafu += "=";
            num += 2 * mul;
        } else if (num + mul).abs() < 2 * f(mul / 5) {
            snafu += "-";
            num += mul;
        } else {
            snafu += "0"
        }
        mul /= 5;
    }
    snafu += match num {
        -2 => "=",
        -1 => "-",
        0 => "0",
        1 => "1",
        2 => "2",
        _ => unreachable!(),
    };
    snafu
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    dbg!(dec_to_snafu(input.lines().map(snafu_to_dec).sum::<i64>()));
}
