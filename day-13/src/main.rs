use std::cmp::Ordering;
use std::fs;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
enum Input {
    Int(i32),
    Vec(Vec<Input>),
}

fn parse_input(input: &mut Chars) -> Input {
    let mut current = Vec::new();
    let mut digits = String::new();
    while let Some(c) = input.next() {
        match c {
            '[' => {
                let inner = parse_input(input);
                current.push(inner);
            }
            ']' => {
                if !digits.is_empty() {
                    current.push(Input::Int(digits.parse().unwrap()));
                    digits.clear();
                }
                return Input::Vec(current);
            }
            ',' => {
                if !digits.is_empty() {
                    current.push(Input::Int(digits.parse().unwrap()));
                    digits.clear();
                }
            }
            _ => {
                digits.push(c);
            }
        }
    }
    panic!("unterminated input");
}

fn parse_line(line: &str) -> Option<Input> {
    let mut chars = line.chars();
    match chars.next() {
        Some('[') => Some(parse_input(&mut chars)),
        None => None,
        _ => panic!("invalid input"),
    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Input::Int(l), Input::Int(r)) => l.cmp(r),
            (Input::Vec(l), Input::Vec(r)) => {
                let mut l = l.iter();
                let mut r = r.iter();
                loop {
                    match (l.next(), r.next()) {
                        (Some(l), Some(r)) => {
                            let result = l.cmp(r);
                            if result != Ordering::Equal {
                                return result;
                            }
                        }
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (Input::Vec(vec), Input::Int(int)) => {
                let mut iter = vec.iter();
                match iter.next() {
                    Some(Input::Int(vec_entry)) => vec_entry.cmp(int).then_with(|| {
                        if iter.next().is_some() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }),
                    Some(nested_vec) => nested_vec.cmp(other),
                    None => Ordering::Less,
                }
            }
            (Input::Int(_), Input::Vec(_)) => other.cmp(self).reverse(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("../13.input").unwrap();

    let mut total_count = 0;
    let mut good_indices = 0;

    for triple in input.lines().collect::<Vec<&str>>().chunks(3) {
        let left = parse_line(triple[0]).unwrap();
        let right = parse_line(triple[1]).unwrap();

        let ordering = left.cmp(&right);

        total_count += 1;
        if ordering != Ordering::Greater {
            good_indices += total_count;
        }
    }

    println!("{}", good_indices);

    let mut all_packets = input.lines().filter_map(parse_line).collect::<Vec<Input>>();

    let decoder_one = parse_line("[[2]]").unwrap();
    let decoder_two = parse_line("[[6]]").unwrap();

    all_packets.push(parse_line("[[2]]").unwrap());
    all_packets.push(parse_line("[[6]]").unwrap());

    all_packets.sort();

    let mut decoder_key = 1;
    for (i, packet) in (1..).zip(all_packets.iter()) {
        if packet == &decoder_one || packet == &decoder_two {
            decoder_key *= i;
        }
    }

    println!("{}", decoder_key);
}
