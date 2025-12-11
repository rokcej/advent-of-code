use std::collections::HashMap;

const ID_START: &str = "svr";
const ID_DAC: &str = "dac";
const ID_FFT: &str = "fft";
const ID_FINISH: &str = "out";

const VISITED_DAC: u8 = 0b01;
const VISITED_FFT: u8 = 0b10;
const VISITED_ALL: u8 = 0b11;

#[derive(Hash, Eq, PartialEq, Clone)]
struct StateKey<'a> {
    id: &'a str,
    visited: u8,
}

struct State<'a> {
    key: StateKey<'a>,
    next_id_index: usize,
    path_count: u64,
}

fn main() {
    let input: String = std::fs::read_to_string("day11/input").expect("Error reading input");

    let map: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (from, to_list) = line.split_once(": ").unwrap();
            return (from, to_list.split_whitespace().collect());
        })
        .collect();

    let start_key = StateKey {
        id: ID_START,
        visited: 0,
    };

    let mut search_stack: Vec<State> = Vec::from([State {
        key: start_key.clone(),
        next_id_index: 0,
        path_count: 0,
    }]);
    let mut path_counts: HashMap<StateKey, u64> = HashMap::new();

    while let Some(state) = search_stack.last_mut() {
        let next_ids = &map[state.key.id];

        if state.next_id_index >= next_ids.len() {
            let state = search_stack.pop().unwrap();
            if let Some(prev_state) = search_stack.last_mut() {
                prev_state.path_count += state.path_count;
            }
            path_counts.insert(state.key, state.path_count);
            continue; // Searched all paths from current state
        }

        let next_id = next_ids[state.next_id_index];
        state.next_id_index += 1;

        if next_id == ID_FINISH {
            if state.key.visited & VISITED_ALL == VISITED_ALL {
                state.path_count += 1;
            }
            continue; // Finished current path
        }

        let mut next_key = StateKey {
            id: next_id,
            visited: state.key.visited,
        };
        if next_id == ID_DAC {
            next_key.visited |= VISITED_DAC;
        } else if next_id == ID_FFT {
            next_key.visited |= VISITED_FFT;
        }

        if let Some(path_count) = path_counts.get(&next_key) {
            state.path_count += path_count;
            continue; // Found cached path count
        }
        
        search_stack.push(State {
            key: next_key,
            next_id_index: 0,
            path_count: 0,
        });
    }

    println!("{}", path_counts.get(&start_key).unwrap());
}
