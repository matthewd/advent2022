use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("../12.input").unwrap();

    let mut start_pos = (0, 0);
    let mut finish_pos = (0, 0);

    let mut heights: HashMap<(i32, i32), i32> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = match c {
                'S' => 0,
                'E' => 25,
                _ => c as i32 - 'a' as i32,
            };
            heights.insert((x as i32, y as i32), height);

            if c == 'E' {
                finish_pos = (x as i32, y as i32);
            }

            if c == 'S' {
                start_pos = (x as i32, y as i32);
            }
        }
    }

    let mut steps: HashMap<(i32, i32), i32> = HashMap::new();
    let mut open: VecDeque<(i32, i32)> = VecDeque::new();

    steps.insert(finish_pos, 0);
    open.push_back(finish_pos);

    while !open.is_empty() {
        let (x, y) = open.pop_front().unwrap();
        let h = heights[&(x, y)];

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(&nh) = heights.get(&(nx, ny)) {
                if !steps.contains_key(&(nx, ny)) && nh >= (h - 1) {
                    steps.insert((nx, ny), steps.get(&(x, y)).unwrap() + 1);
                    open.push_back((nx, ny));
                }
            }
        }
    }

    println!("{}", steps[&start_pos]);

    let shortest_path = heights
        .keys()
        .filter(|(x, y)| heights[&(*x, *y)] == 0)
        .filter(|(x, y)| steps.contains_key(&(*x, *y)))
        .map(|(x, y)| steps[&(*x, *y)])
        .min()
        .unwrap();

    println!("{}", shortest_path);
}
