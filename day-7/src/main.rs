use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../7.input").unwrap();

    let mut cwd: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, Vec<String>> = HashMap::new();
    let mut files: HashMap<String, i64> = HashMap::new();

    dirs.insert("/".to_string(), Vec::new());

    let mut stream = input.lines().peekable();

    while let Some(line) = stream.next() {
        let mut parts = line.split_whitespace();
        parts.next();
        match parts.next() {
            Some("cd") => match parts.next() {
                Some("..") => {
                    cwd.pop();
                }
                Some("/") => {
                    cwd.clear();
                }
                Some(dir) => {
                    cwd.push(dir.to_string());
                }
                _ => {
                    panic!("Invalid cd");
                }
            },
            Some("ls") => {
                let path = "/".to_string() + &cwd.join("/");
                let mut path_prefix = path.clone();
                if path_prefix.len() > 1 {
                    path_prefix.push('/');
                }
                while let Some(entry) = stream.peek() {
                    if entry.starts_with("$") {
                        break;
                    }

                    let mut parts = stream.next().unwrap().split_whitespace();
                    let dir_or_size = parts.next().unwrap();
                    let name = parts.next().unwrap();

                    let full_name = path_prefix.clone() + name;

                    dirs.get_mut(&path).unwrap().push(name.to_string());
                    if dir_or_size == "dir" {
                        dirs.entry(full_name).or_insert(Vec::new());
                    } else {
                        files.insert(full_name, dir_or_size.parse().unwrap());
                    }
                }
            }
            _ => {
                panic!("Invalid command");
            }
        }
    }

    let mut total_sizes: HashMap<String, i64> = HashMap::new();

    fn total_size(
        total_sizes: &mut HashMap<String, i64>,
        dirs: &HashMap<String, Vec<String>>,
        files: &HashMap<String, i64>,
        dir: String,
    ) -> i64 {
        if let Some(size) = total_sizes.get(&dir) {
            return *size;
        }

        let mut total = 0;
        for entry in dirs.get(&dir).unwrap() {
            let full_name = if dir == "/" {
                "/".to_string() + entry
            } else {
                dir.clone() + "/" + entry
            };
            if dirs.contains_key(&full_name) {
                total += total_size(total_sizes, dirs, files, full_name);
            } else {
                total += files.get(&full_name).unwrap();
            }
        }
        total_sizes.insert(dir, total);
        total
    }

    // force calculate all sizes
    let grand_total = total_size(&mut total_sizes, &dirs, &files, "/".to_string());

    let mut sum_of_interesting_sizes = 0;
    for (_dir, &size) in &total_sizes {
        if size < 100000 {
            sum_of_interesting_sizes += size;
        }
    }

    println!("{}", sum_of_interesting_sizes);

    let capacity = 70000000;
    let space_required = 30000000;
    let amount_to_free = grand_total - capacity + space_required;

    let mut smallest_candidate = capacity;

    for (_dir, size) in total_sizes {
        if size < smallest_candidate && size >= amount_to_free {
            smallest_candidate = size;
        }
    }

    println!("{}", smallest_candidate);
}
