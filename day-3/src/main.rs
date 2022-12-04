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
        res |= (1 as i64) << (priority(c) - 1);
    }
    res
}

fn main() {
    let input = fs::read_to_string("../3.input").unwrap();

    let mut priority_sum = 0;

    for line in input.lines() {
        let compartments = [line[0..line.len() / 2].to_string(), line[line.len() / 2..].to_string()];
        let commons = compartments[0].chars().filter(|c| compartments[1].contains(*c)).collect::<Vec<char>>();
        let common = commons.first().unwrap();

        priority_sum += priority(*common);
    }

    println!("{}", priority_sum);

    let packs = input.lines().map(|line| pack(line));
    let mut iter = packs.into_iter();

    let mut priority_sum = 0;

    while let Some(p1) = iter.next() {
        let p2 = iter.next().unwrap();
        let p3 = iter.next().unwrap();

        let common = p1 & p2 & p3;
        let priority = common.trailing_zeros() + 1;
        priority_sum += priority as i32;
    }

    println!("{}", priority_sum);
}
