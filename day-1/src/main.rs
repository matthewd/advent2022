use std::fs;

fn main() {
    let input = fs::read_to_string("../1.input").unwrap();

    // Ruby: input.lines.slice_after { |line| line == "" }.map { |group| group.map(&:to_i) }

    let mut groups = Vec::new();

    let mut group = Vec::new();
    for line in input.lines() {
        if line == "" {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.parse().unwrap());
        }
    }

    let mut sums: Vec<_> = groups.into_iter().map(|group| group.iter().sum()).collect();
    sums.sort();

    let max = sums.iter().max().unwrap();

    println!("{}", max);

    let sum_of_top_3: i32 = sums.iter().rev().take(3).sum();

    println!("{}", sum_of_top_3);
}
