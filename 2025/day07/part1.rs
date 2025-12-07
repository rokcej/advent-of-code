fn main() {
    let input: String = std::fs::read_to_string("day07/input").expect("Error reading input");

    let mut start_pos: Option<(i32, i32)> = None;
    let grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    'S' => {
                        start_pos = Some((y as i32, x as i32));
                        return false;
                    }
                    '^' => return true,
                    '.' => return false,
                    _ => panic!("Invalid character {ch}"),
                })
                .collect()
        })
        .collect();

    let start_position: (i32, i32) = start_pos.expect("Missing start position");
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut position_stack = vec![start_position];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];
    let mut split_count: u64 = 0;

    while let Some((y, x)) = position_stack.pop() {
        if y < 0 || y >= height || x < 0 || x >= width {
            continue;
        }
        let (uy, ux) = (y as usize, x as usize);

        if visited[uy][ux] {
            continue;
        }
        visited[uy][ux] = true;

        if grid[uy][ux] {
            position_stack.push((y + 1, x - 1));
            position_stack.push((y + 1, x + 1));
            split_count += 1;
        } else {
            position_stack.push((y + 1, x));
        }
    }

    println!("{split_count}");
}
