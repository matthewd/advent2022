use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../8.input").unwrap();

    let height_grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut visibility_map: HashMap<(usize, usize), bool> = HashMap::new();

    let column_count = input.lines().next().unwrap().len();

    height_grid.iter().enumerate().for_each(|(y, row)| {
        let mut highest_seen = -1;
        row.iter().enumerate().for_each(|(x, c)| {
            let digit = *c;
            if digit > highest_seen {
                highest_seen = digit;
                visibility_map.insert((x, y), true);
            }
        });

        let mut highest_seen = -1;
        row.iter().rev().enumerate().for_each(|(xx, c)| {
            let x = column_count - xx - 1;
            let digit = *c;
            if digit > highest_seen {
                highest_seen = digit;
                visibility_map.insert((x, y), true);
            }
        });
    });

    let columns: Vec<Vec<i32>> = (0..column_count)
        .map(|x| height_grid.iter().map(|row| row[x]).collect())
        .collect();

    columns.iter().enumerate().for_each(|(x, column)| {
        let mut highest_seen = -1;
        column.iter().enumerate().for_each(|(y, c)| {
            let digit = *c;
            if digit > highest_seen {
                highest_seen = digit;
                visibility_map.insert((x, y), true);
            }
        });

        let mut highest_seen = -1;
        column.iter().rev().enumerate().for_each(|(yy, c)| {
            let y = height_grid.len() - yy - 1;
            let digit = *c;
            if digit > highest_seen {
                highest_seen = digit;
                visibility_map.insert((x, y), true);
            }
        });
    });

    height_grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if visibility_map.get(&(x, y)).is_none() {
                print!(" ");
            } else {
                print!("{}", c);
            }
        });
        println!();
    });

    println!("{}", visibility_map.values().filter(|&v| *v).count());

    let height_map: HashMap<(usize, usize), i32> = height_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .collect();

    fn visible_straight_line_of_sight(
        (x, y): (usize, usize),
        (dx, dy): (i32, i32),
        height_map: &HashMap<(usize, usize), i32>,
    ) -> i32 {
        let mut count = 0;
        let own_height = height_map.get(&(x, y)).unwrap();
        let mut x = x as i32;
        let mut y = y as i32;
        loop {
            x += dx;
            y += dy;

            let height = match height_map.get(&(x as usize, y as usize)) {
                Some(h) => h,
                None => break,
            };

            count += 1;

            if height >= own_height {
                break;
            }
        }
        count
    }

    let mut max_visible = 0;

    for (home_x, home_y) in height_map.keys() {
        let mut visible = 1;
        for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            visible *= visible_straight_line_of_sight((*home_x, *home_y), (dx, dy), &height_map);
        }
        if visible > max_visible {
            max_visible = visible;
        }
    }

    println!("{}", max_visible);
}
