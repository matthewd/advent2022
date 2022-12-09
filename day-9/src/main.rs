use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../9.input").unwrap();

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    // position for head and tail
    let (mut hx, mut hy): (i32, i32) = (0, 0);
    let (mut tx, mut ty): (i32, i32) = (0, 0);

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let count = parts[1].parse::<i32>().unwrap();

        for _ in 0..count {
            match direction {
                "U" => hy += 1,
                "D" => hy -= 1,
                "L" => hx -= 1,
                "R" => hx += 1,
                _ => panic!("Unknown direction"),
            }

            // if head and tail are not adjacent (including diagonals), move tail
            if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                // if in a straight line, just move closer
                if hx == tx {
                    if hy > ty {
                        ty += 1;
                    } else {
                        ty -= 1;
                    }
                } else if hy == ty {
                    if hx > tx {
                        tx += 1;
                    } else {
                        tx -= 1;
                    }
                } else {
                    // if in a diagonal, move both closer
                    if hx > tx {
                        tx += 1;
                    } else {
                        tx -= 1;
                    }

                    if hy > ty {
                        ty += 1;
                    } else {
                        ty -= 1;
                    }
                }
            }

            visited.insert((tx, ty), true);
        }
    }

    println!("{}", visited.len());

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    // position for head and tail
    let mut positions: Vec<(i32, i32)> = Vec::new();

    for _ in 0..=9 {
        positions.push((0, 0));
    }

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let count = parts[1].parse::<i32>().unwrap();

        for _ in 0..count {
            match direction {
                "U" => positions[0].1 += 1,
                "D" => positions[0].1 -= 1,
                "L" => positions[0].0 -= 1,
                "R" => positions[0].0 += 1,
                _ => panic!("Unknown direction"),
            }

            fn follow_point((hx, hy): (i32, i32), (mut tx, mut ty): (i32, i32)) -> (i32, i32) {
                // if head and tail are not adjacent (including diagonals), move tail
                if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                    // if in a straight line, just move closer
                    if hx == tx {
                        if hy > ty {
                            ty += 1;
                        } else {
                            ty -= 1;
                        }
                    } else if hy == ty {
                        if hx > tx {
                            tx += 1;
                        } else {
                            tx -= 1;
                        }
                    } else {
                        // if in a diagonal, move both closer
                        if hx > tx {
                            tx += 1;
                        } else {
                            tx -= 1;
                        }

                        if hy > ty {
                            ty += 1;
                        } else {
                            ty -= 1;
                        }
                    }
                }

                return (tx, ty);
            }

            for i in 1..positions.len() {
                positions[i] = follow_point(positions[i - 1], positions[i]);
            }

            visited.insert(positions[positions.len() - 1], true);
        }
    }

    println!("{}", visited.len());
}
