use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../14.input").unwrap();

    let mut rocks: HashMap<(i32, i32), char> = HashMap::new();

    for line in input.lines() {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        for p in points.windows(2) {
            let parts = p[0].split(",").collect::<Vec<&str>>();
            let mut sx = parts[0].parse::<i32>().unwrap();
            let mut sy = parts[1].parse::<i32>().unwrap();

            let parts = p[1].split(",").collect::<Vec<&str>>();
            let mut ex = parts[0].parse::<i32>().unwrap();
            let mut ey = parts[1].parse::<i32>().unwrap();

            if sx > ex {
                (sx, ex) = (ex, sx);
            }
            if sy > ey {
                (sy, ey) = (ey, sy);
            }

            for x in sx..=ex {
                for y in sy..=ey {
                    rocks.insert((x, y), '#');
                }
            }
        }
    }

    let max_y = rocks.keys().map(|(_, y)| y).max().unwrap();

    fn settle(rocks: &HashMap<(i32, i32), char>, floor: Option<i32>, void: Option<i32>) -> HashMap<(i32, i32), char> {
        let mut settled: HashMap<(i32, i32), char> = HashMap::new();

        'sand: loop {
            let (mut x, mut y) = (500, 0);

            'falling: loop {
                if void != None && y > void.unwrap() {
                    break 'sand;
                }

                if Some(y + 1) == floor {
                    settled.insert((x, y), 'o');
                    break 'falling;
                } else if !rocks.contains_key(&(x, y + 1)) && !settled.contains_key(&(x, y + 1)) {
                    y += 1;
                } else if !rocks.contains_key(&(x - 1, y + 1)) && !settled.contains_key(&(x - 1, y + 1)) {
                    x -= 1;
                    y += 1;
                } else if !rocks.contains_key(&(x + 1, y + 1)) && !settled.contains_key(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                } else {
                    settled.insert((x, y), 'o');
                    break 'falling;
                }
            }

            if (x, y) == (500, 0) {
                break 'sand;
            }
        }

        settled
    }

    fn draw(rocks: &HashMap<(i32, i32), char>, settled: &HashMap<(i32, i32), char>, floor: Option<i32>, void: Option<i32>) {
        let settled_min_x = settled.keys().map(|(x, _)| x).min().or(Some(&500)).unwrap();
        let rocks_min_x = rocks.keys().map(|(x, _)| x).min().unwrap();

        let min_x = settled_min_x.min(rocks_min_x);

        let settled_max_x = settled.keys().map(|(x, _)| x).max().or(Some(&500)).unwrap();
        let rocks_max_x = rocks.keys().map(|(x, _)| x).max().unwrap();

        let max_x = settled_max_x.max(rocks_max_x);

        let max_y = floor.or(void).unwrap();

        for y in 0..=max_y {
            for x in *min_x..=*max_x {
                if y == 0 && x == 500 {
                    print!("@");
                } else if rocks.contains_key(&(x, y)) {
                    print!("{}", rocks.get(&(x, y)).unwrap());
                } else if settled.contains_key(&(x, y)) {
                    print!("{}", settled.get(&(x, y)).unwrap());
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    let part1 = settle(&rocks, None, Some(*max_y));

    println!("{}", part1.len());

    let part2 = settle(&rocks, Some(max_y + 2), None);

    println!("{}", part2.len());
}
