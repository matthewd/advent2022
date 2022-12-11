use num_integer::lcm;
use std::collections::VecDeque;
use std::fs;

struct Monkey {
    items: VecDeque<i64>,
    count: usize,
    operation_m: i64,
    operation_c: i64,
    test: i64,
    when_true: i64,
    when_false: i64,
}

fn main() {
    let input = fs::read_to_string("../11.input").unwrap();

    for (rounds, relaxation) in [(20, 3), (10000, 1)] {
        let mut monkeys: Vec<Monkey> = Vec::new();

        let mut multiple = 1;

        for paragraph in input.split("\n\n") {
            let mut tokens = paragraph.split_whitespace();
            tokens.next(); // "Monkey"
            tokens.next(); // "n:"
            tokens.next(); // "Starting"
            tokens.next(); // "items:"

            let mut items = VecDeque::new();

            while let Some(token) = tokens.next() {
                if token == "Operation:" {
                    break;
                }
                // trim trailing comma
                let token = token.trim_end_matches(',');
                let item = token.parse::<i64>().unwrap();
                items.push_back(item);
            }

            tokens.next(); // "new"
            tokens.next(); // "="
            tokens.next(); // "old"
            let mut operator = tokens.next().unwrap(); // "+" or "*"
            let operand_str = tokens.next().unwrap(); // "1" or "2" or "old"
            let operand;

            if operand_str == "old" {
                if operator == "+" {
                    operator = "*";
                    operand = 2;
                } else {
                    operand = 0;
                }
            } else {
                operand = operand_str.parse::<i64>().unwrap();
            }

            tokens.next(); // "Test:"
            tokens.next(); // "divisible"
            tokens.next(); // "by"
            let divisor = tokens.next().unwrap().parse::<i64>().unwrap();
            multiple = lcm(multiple, divisor);

            tokens.next(); // "If"
            tokens.next(); // "true:"
            tokens.next(); // "throw"
            tokens.next(); // "to"
            tokens.next(); // "monkey"
            let true_target = tokens.next().unwrap().parse::<i64>().unwrap();

            tokens.next(); // "If"
            tokens.next(); // "false:"
            tokens.next(); // "throw"
            tokens.next(); // "to"
            tokens.next(); // "monkey"
            let false_target = tokens.next().unwrap().parse::<i64>().unwrap();

            let monkey = Monkey {
                items,
                count: 0,
                operation_m: match operator {
                    "+" => 1,
                    "*" => operand,
                    _ => panic!("Unknown operator"),
                },
                operation_c: match operator {
                    "+" => operand,
                    "*" => 0,
                    _ => panic!("Unknown operator"),
                },
                test: divisor,
                when_true: true_target,
                when_false: false_target,
            };
            monkeys.push(monkey);
        }

        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                while let Some(mut item) = monkeys[i].items.pop_front() {
                    // Inspect
                    if monkeys[i].operation_m == 0 && monkeys[i].operation_c == 0 {
                        item *= item;
                    } else {
                        item = item * monkeys[i].operation_m + monkeys[i].operation_c;
                    }

                    // Relax
                    item /= relaxation;
                    item %= multiple;

                    // Test
                    let target = if item % monkeys[i].test == 0 {
                        monkeys[i].when_true
                    } else {
                        monkeys[i].when_false
                    };

                    // Throw
                    monkeys[target as usize].items.push_back(item);

                    // Count
                    monkeys[i].count += 1;
                }
            }
        }

        let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
        counts.sort();
        let highest_two = counts.iter().rev().take(2).collect::<Vec<_>>();
        let monkey_business = highest_two[0] * highest_two[1];

        println!("{}", monkey_business);
    }
}
