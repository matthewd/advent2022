use std::fs;

fn main() {
    let input = fs::read_to_string("../5.input").unwrap();

    let (stackstr, movestr) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut indexes: Vec<usize> = Vec::new();

    for line in stackstr.lines().rev() {
        if indexes.is_empty() {
            if line.chars().nth(0).unwrap() == ' ' {
                continue;
            }

            for (i, _) in line.match_indices('[') {
                indexes.push(i + 1);
                stacks.push(Vec::new());
            }
        }

        for (stack, index) in stacks.iter_mut().zip(indexes.iter()) {
            let c = line.chars().nth(*index).unwrap();
            if c != ' ' {
                stack.push(c);
            }
        }
    }

    let mut moves: Vec<(usize, usize, usize)> = Vec::new();

    for line in movestr.lines() {
        // line is of the form "move <num> from <num> to <num>"
        let mut parts = line.split_whitespace();
        parts.next(); // skip "move"
        let moving: usize = parts.next().unwrap().parse().unwrap();
        parts.next(); // skip "from"
        let from: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        parts.next(); // skip "to"
        let to: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        moves.push((moving, from, to));
    }

    let mut result_stacks: Vec<_> = stacks.iter().map(|s| s.clone()).collect();

    for (moving, from, to) in moves.iter() {
        // println!("Moving {} from {} to {}", moving, from, to);
        // println!("Before: {:?} {:?}", result_stacks[from], result_stacks[to]);

        for _ in 0..*moving {
            let c = result_stacks[*from].pop().unwrap();
            result_stacks[*to].push(c);
        }

        // println!("After: {:?} {:?}", result_stacks[from], result_stacks[to]);
    }

    for stack in result_stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

    let mut result_stacks: Vec<_> = stacks.iter().map(|s| s.clone()).collect();

    for (moving, from, to) in moves {
        // println!("Moving {} from {} to {}", moving, from, to);
        // println!("Before: {:?} {:?}", result_stacks[from], result_stacks[to]);

        let index = result_stacks[from].len() - moving;
        let crates = result_stacks[from].split_off(index);
        result_stacks[to].extend(crates);

        // println!("After: {:?} {:?}", result_stacks[from], result_stacks[to]);
    }

    for stack in result_stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
