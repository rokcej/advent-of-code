// Pack global recursion parameters into struct to avoid repetition
struct Data<'a> {
    height: i32,
    width: i32,
    grid: &'a Vec<Vec<bool>>,
    count_cache: &'a mut Vec<Vec<Option<u64>>>,
}

fn get_timeline_count(y: i32, x: i32, data: &mut Data) -> u64 {
    if y < 0 || y >= data.height || x < 0 || x >= data.width {
        return 1;
    }

    let (uy, ux) = (y as usize, x as usize);
    if let Some(count) = data.count_cache[uy][ux] {
        return count;
    }

    let mut count: u64 = 0;
    if data.grid[uy][ux] {
        count += get_timeline_count(y + 1, x - 1, data);
        count += get_timeline_count(y + 1, x + 1, data);
    } else {
        count += get_timeline_count(y + 1, x, data);
    }
    data.count_cache[uy][ux] = Some(count);
    return count;
}

fn main() {
    let input: String = std::fs::read_to_string("day07/input").expect("Error reading input");

    let mut start_position: Option<(i32, i32)> = None;
    let grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| match character {
                    'S' => {
                        start_position = Some((y as i32, x as i32));
                        return false;
                    }
                    '^' => return true,
                    '.' => return false,
                    _ => panic!("Invalid character {character}"),
                })
                .collect()
        })
        .collect();

    let (y, x): (i32, i32) = start_position.expect("Missing start position");
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut count_cache: Vec<Vec<Option<u64>>> = vec![vec![None; width as usize]; height as usize];

    let mut data = Data {
        height,
        width,
        grid: &grid,
        count_cache: &mut count_cache,
    };
    let timeline_count = get_timeline_count(y, x, &mut data);
    println!("{timeline_count}");
}
