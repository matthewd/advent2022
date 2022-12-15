use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::ops::Range;

fn ranges_at_y(
    target_y: i32,
    sensors: &Vec<(i32, i32, i32)>,
    clamp_x: Option<Range<i32>>,
) -> Vec<Range<i32>> {
    let mut ranges: Vec<Range<i32>> = Vec::new();
    for (x, y, distance) in sensors {
        let y_offset = (target_y - y).abs();

        if y_offset > *distance {
            continue;
        }

        let x_offset = distance - y_offset;

        let mut range = Range {
            start: x - x_offset,
            end: x + x_offset + 1,
        };

        if let Some(ref clamp_x) = clamp_x {
            if range.end < clamp_x.start || range.start > clamp_x.end {
                continue;
            }

            if range.start < clamp_x.start {
                range.start = clamp_x.start;
            }

            if range.end > clamp_x.end {
                range.end = clamp_x.end;
            }
        }

        ranges.push(range);
    }
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    ranges
}

fn flatten_ranges(ranges: Vec<Range<i32>>) -> Vec<Range<i32>> {
    let mut result: Vec<Range<i32>> = Vec::new();

    let mut last_end = ranges[0].start - 1;
    for range in ranges {
        if range.start > last_end {
            last_end = range.end;

            result.push(range);
        } else if range.end > last_end {
            last_end = range.end;

            result.last_mut().unwrap().end = range.end;
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("../15.input").unwrap();
    let target_row = 2000000;
    let search_max = 4000000;
    let coord_mult = 4000000;

    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();
    let mut beacons_on_target: HashSet<i32> = HashSet::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let r = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let caps = r.captures(&line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let bx = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let by = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

        let distance = (bx - x).abs() + (by - y).abs();

        beacons.insert((bx, by));
        if by == target_row {
            beacons_on_target.insert(bx);
        }

        sensors.push((x, y, distance));
    }

    let ranges = ranges_at_y(target_row, &sensors, None);
    let flattened_ranges = flatten_ranges(ranges);

    let count = flattened_ranges
        .iter()
        .map(|r| r.end - r.start)
        .sum::<i32>();

    println!("{}", count - beacons_on_target.len() as i32);

    for target_row in 0..=search_max {
        let ranges = ranges_at_y(target_row, &sensors, Some(0..search_max));
        let flattened_ranges = flatten_ranges(ranges);

        if flattened_ranges.len() == 1
            && flattened_ranges[0].start <= 0
            && flattened_ranges[0].end >= search_max
        {
            continue;
        }

        let target_col = flattened_ranges[0].end;
        println!(
            "({}, {}) -> {}",
            target_col,
            target_row,
            target_col as i64 * coord_mult + target_row as i64
        );
        break;
    }
}
