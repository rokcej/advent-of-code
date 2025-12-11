use std::collections::HashMap;

const ID_START: &str = "you";
const ID_FINISH: &str = "out";

fn main() {
    let input: String = std::fs::read_to_string("day11/input").expect("Error reading input");

    let map: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (from, to_list) = line.split_once(": ").unwrap();
            return (from, to_list.split_whitespace().collect());
        })
        .collect();

    let mut search_stack: Vec<&str> = Vec::from([ID_START]);
    let mut path_count: u64 = 0;

    while let Some(id) = search_stack.pop() {
        if id == ID_FINISH {
            path_count += 1;
            continue;
        }

        if let Some(next_list) = map.get(id) {
            search_stack.extend(next_list);
        }
    }

    println!("{path_count}");
}
