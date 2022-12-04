use std::fs;

fn main() {
    let input = fs::read_to_string("../4.input").unwrap();

    let mut fully_contained_count = 0;
    let mut partial_overlap_count = 0;

    for line in input.lines() {
        // line contains a pair of assignments: "3-18,12-15"

        let assignments = line.split(',').map(|assigned_range| {
            // assigned_range is something like "3-18"
            let mut parts = assigned_range.split('-');
            let start = parts.next().unwrap().parse::<u32>().unwrap();
            let finish = parts.next().unwrap().parse::<u32>().unwrap();
            (start, finish)
        }).collect::<Vec<_>>();

        let full_overlap = fully_contained(assignments[0], assignments[1]) ||
            fully_contained(assignments[1], assignments[0]);

        if full_overlap {
            fully_contained_count += 1;
        }

        if partially_overlapping(assignments[0], assignments[1]) ||
            partially_overlapping(assignments[1], assignments[0]) {
            partial_overlap_count += 1;
        }
    }

    println!("{}", fully_contained_count);
    println!("{}", partial_overlap_count);
}

fn fully_contained(outer: (u32, u32), inner: (u32, u32)) -> bool {
    inner.0 >= outer.0 && inner.1 <= outer.1
}

fn fully_separated(outer: (u32, u32), inner: (u32, u32)) -> bool {
    // the two ranges are fully separated if the end of the first is before the start of the second or vice versa
    outer.1 < inner.0 || outer.0 > inner.1
}

fn partially_overlapping(outer: (u32, u32), inner: (u32, u32)) -> bool {
    !fully_separated(outer, inner)
}
