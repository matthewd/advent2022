use std::fs;

fn priority(c: char) -> i32 {
    match c {
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        _ => panic!("Invalid character"),
    }
}

fn pack(s: &str) -> i64 {
    let mut res = 0;
    for c in s.chars() {
        res |= (1 as i64) << priority(c);
    }
    res
}

fn main() {
    let input = fs::read_to_string("../3.input").unwrap();

    let mut priority_sum = 0;

    for line in input.lines() {
        let (left, right) = (
            line[0..line.len() / 2].to_string(),
            line[line.len() / 2..].to_string(),
        );

        if let Some(common) = left.chars().find(|c| right.contains(*c)) {
            priority_sum += priority(common);
        }
    }

    println!("{}", priority_sum);

    let mut priority_sum = 0;

    let packs = input.lines().map(|line| pack(line));
    let mut iter = packs.into_iter();
    while let Some(p1) = iter.next() {
        let p2 = iter.next().unwrap();
        let p3 = iter.next().unwrap();

        let common = p1 & p2 & p3;
        priority_sum += common.trailing_zeros();
    }

    println!("{}", priority_sum);
}
