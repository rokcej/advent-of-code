#[derive(Copy, Clone)]
enum Operation {
    Add,
    Mul,
}

fn try_parse_operation(character: char) -> Option<Operation> {
    match character {
        '+' => return Some(Operation::Add),
        '*' => return Some(Operation::Mul),
        _ => return None,
    }
}

fn apply_operation(op: Operation, a: u64, b: u64) -> u64 {
    match op {
        Operation::Add => return a + b,
        Operation::Mul => return a * b,
    }
}

fn main() {
    let input: String = std::fs::read_to_string("day06/input").expect("Error reading input");
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let ops: Vec<(usize, Operation)> = lines
        .last()
        .expect("Missing operations")
        .chars()
        .enumerate()
        .filter_map(|(i, character)| {
            if let Some(op) = try_parse_operation(character) {
                return Some((i, op));
            }
            return None;
        })
        .collect();

    let number_grid = &lines[..lines.len() - 1];
    let column_count: usize = number_grid
        .iter()
        .map(|row| row.chars().count())
        .max()
        .expect("Missing numbers");

    let mut grand_total: u64 = 0;
    for (i_op, &(i_start, op)) in ops.iter().enumerate() {
        let i_end: usize = if (i_op + 1) < ops.len() {
            ops[i_op + 1].0 - 1
        } else {
            column_count - 1
        };

        let mut op_result: Option<u64> = None;
        for i_column in i_start..=i_end {
            let column: String = number_grid
                .iter()
                .filter_map(|row| {
                    let character = row.chars().nth(i_column).expect("Missing column");
                    if character.is_ascii_digit() {
                        return Some(character);
                    }
                    return None;
                })
                .collect();

            if column.chars().count() > 0 {
                let column_number: u64 = column.parse().expect("Invalid number");
                if let Some(op_result_value) = op_result {
                    op_result = Some(apply_operation(op, op_result_value, column_number));
                } else {
                    op_result = Some(column_number);
                }
            }
        }
        grand_total += op_result.expect("Empty operation result");
    }
    println!("{grand_total}");
}
