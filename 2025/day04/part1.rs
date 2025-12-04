fn char_to_paper(input_char: char) -> bool {
    match input_char {
        '@' => return true,
        '.' => return false,
        _ => panic!("Invalid character {input_char}"),
    };
}

fn get_neighbor_count(grid: &Vec<Vec<bool>>, i_row: isize, i_col: isize) -> u8 {
    let mut count: u8 = 0;

    for y in i_row - 1..=i_row + 1 {
        let Some(row) = grid.get(y as usize) else {
            continue;
        };
        for x in i_col - 1..=i_col + 1 {
            if y == i_row && x == i_col {
                continue;
            }
            if let Some(&value) = row.get(x as usize) {
                if value {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn main() {
    let input: String = std::fs::read_to_string("day04/input").expect("Error reading input");
    let paper_grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(char_to_paper).collect())
        .collect();

    let mut accessible_paper_count: u32 = 0;
    for (i_row, paper_row) in paper_grid.iter().enumerate() {
        for (i_col, &paper) in paper_row.iter().enumerate() {
            if paper && get_neighbor_count(&paper_grid, i_row as isize, i_col as isize) < 4 {
                accessible_paper_count += 1;
            }
        }
    }
    println!("{accessible_paper_count}");
}
