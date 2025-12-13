use std::collections::VecDeque;

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
}

struct SearchState {
    lights: Vec<bool>,
    button_press_count: u64,
    i_first_button: usize,
}

fn try_parse_lights(token: &str) -> Option<Vec<bool>> {
    let i_open = token.find('[')?;
    let i_close = token.find(']')?;
    return Some(
        token[i_open + 1..i_close]
            .chars()
            .map(|ch| ch == '#')
            .collect(),
    );
}

fn try_parse_button(token: &str) -> Option<Vec<u8>> {
    let i_open = token.find('(')?;
    let i_close = token.find(')')?;
    return Some(
        token[i_open + 1..i_close]
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect(),
    );
}

fn parse_machine(line: &str) -> Machine {
    let mut target_lights: Option<Vec<bool>> = None;
    let mut buttons: Vec<Vec<u8>> = Vec::new();

    for token in line.split_whitespace() {
        if let Some(lights) = try_parse_lights(token) {
            target_lights = Some(lights);
        } else if let Some(button) = try_parse_button(token) {
            buttons.push(button);
        }
    }

    return Machine {
        target_lights: target_lights.unwrap(),
        buttons,
    };
}

fn get_button_press_count(line: &str) -> u64 {
    let machine = parse_machine(line);

    let mut bfs_queue: VecDeque<SearchState> = VecDeque::from([SearchState {
        lights: vec![false; machine.target_lights.len()],
        button_press_count: 0,
        i_first_button: 0,
    }]);

    while let Some(state) = bfs_queue.pop_front() {
        if state.lights == machine.target_lights {
            return state.button_press_count;
        }

        // Potential improvement: Add check for "already seen" light states

        for i_button in state.i_first_button..machine.buttons.len() {
            let mut new_lights = state.lights.clone();
            for &i_light in &machine.buttons[i_button] {
                let i_light = i_light as usize;
                new_lights[i_light] = !new_lights[i_light];
            }

            bfs_queue.push_back(SearchState {
                lights: new_lights,
                button_press_count: state.button_press_count + 1,
                i_first_button: i_button,
            });
        }
    }

    panic!("No solution possible");
}

fn main() {
    let input: String = std::fs::read_to_string("day10/input").expect("Error reading input");
    let button_press_sum: u64 = input.lines().map(get_button_press_count).sum();
    println!("{button_press_sum}");
}
