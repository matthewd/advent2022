use std::fs;

fn main() {
    let input = fs::read_to_string("../1.input").unwrap();

    // Ruby: input.lines.slice_after { |line| line == "" }.map { |group| group.map(&:to_i) }

    let groups = input
        .lines()
        .fold(vec![vec![]], |mut groups, line| {
            if line == "" {
                groups.push(vec![]);
            } else {
                groups.last_mut().unwrap().push(line.parse::<i32>().unwrap());
            }
            groups
        });

    let mut sums = groups
        .iter()
        .map(|group| group.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    let mut sorted_sums = &mut sums[..];
    sorted_sums.sort();

    let max = sorted_sums.iter().max().unwrap();

    let top_3 = sorted_sums
        .iter()
        .rev()
        .take(3)
        .collect::<Vec<&i32>>();

    println!("{}", max);

    let sum_of_top_3 = top_3[0] + top_3[1] + top_3[2];
    // or:
    // let sum_of_top_3 = top_3.iter().sum::<i32>();

    println!("{}", sum_of_top_3);
}
