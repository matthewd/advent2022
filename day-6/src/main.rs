use std::fs;

fn main() {
    let input = fs::read_to_string("../6.input").unwrap();

    // ruby: input.each_cons(4).index { |a, b, c, d| [a, b, c, d].uniq.size == 4 }

    for (i, chars) in input.chars().collect::<Vec<_>>().windows(4).enumerate() {
        if chars.iter().collect::<std::collections::HashSet<_>>().len() == chars.len() {
            println!("{}", i + chars.len());
            break;
        }
    }

    for (i, chars) in input.chars().collect::<Vec<_>>().windows(14).enumerate() {
        if chars.iter().collect::<std::collections::HashSet<_>>().len() == chars.len() {
            println!("{}", i + chars.len());
            break;
        }
    }
}
