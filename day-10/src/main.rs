use std::fs;

fn main() {
    let input = fs::read_to_string("../10.input").unwrap();

    let mut cycle_count: i32 = 0;
    let mut previous_cycle = 0;
    let mut register = 1;

    let mut report_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut pixels = vec![' '; 240];

    let mut signal_sum = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let operation = parts.next().unwrap();

        let change;

        match operation {
            "noop" => {
                change = 0;
                cycle_count += 1;
            }
            "addx" => {
                let value = parts.next().unwrap().parse::<i32>().unwrap();

                cycle_count += 2;

                change = value;
            }
            _ => panic!("Unknown operation: {}", operation),
        }

        if !report_cycles.is_empty() && cycle_count >= report_cycles[0] {
            let signal = report_cycles[0] * register;
            signal_sum += signal;

            report_cycles.remove(0);
        }

        for cycle in previous_cycle..cycle_count {
            let column = cycle % 40;

            if (register - column).abs() < 2 {
                pixels[cycle as usize] = '#';
            }
        }

        register += change;
        previous_cycle = cycle_count;
    }

    println!("{}", signal_sum);

    for row in pixels.chunks(40) {
        println!("{}", row.iter().collect::<String>());
    }
}
