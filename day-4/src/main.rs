use std::fs;

fn main() {
    let input = fs::read_to_string("../4.input").unwrap();

    let mut fully_contained_count = 0;
    let mut partial_overlap_count = 0;

    for line in input.lines() {
        // line contains a pair of assignments: "3-18,12-15"

        let assignments: Vec<(u32, u32)> = line
            .split(',')
            .map(|assigned_range| {
                // assigned_range is something like "3-18"
                let (start, finish) = assigned_range.split_once('-').unwrap();
                (start.parse().unwrap(), finish.parse().unwrap())
            })
            .collect();

        let full_overlap = fully_contained(assignments[0], assignments[1])
            || fully_contained(assignments[1], assignments[0]);

        if full_overlap {
            fully_contained_count += 1;
        }

        if partially_overlapping(assignments[0], assignments[1]) {
            partial_overlap_count += 1;
        }
    }

    println!("{}", fully_contained_count);
    println!("{}", partial_overlap_count);
}

fn fully_contained(outer: (u32, u32), inner: (u32, u32)) -> bool {
    inner.0 >= outer.0 && inner.1 <= outer.1
}

fn partially_overlapping(left: (u32, u32), right: (u32, u32)) -> bool {
    !(left.1 < right.0 || left.0 > right.1)
}
