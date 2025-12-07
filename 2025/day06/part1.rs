#[derive(Copy, Clone)]
enum Operation {
    Add,
    Mul,
}

fn parse_operation(string: &str) -> Operation {
    match string {
        "+" => return Operation::Add,
        "*" => return Operation::Mul,
        _ => panic!("Invalid operation {string}"),
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

    let number_grid: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|number| number.parse().expect("Invalid number"))
                .collect()
        })
        .collect();

    let ops: Vec<Operation> = lines
        .last()
        .expect("Missing operations")
        .trim()
        .split_whitespace()
        .map(parse_operation)
        .collect();

    let mut grand_total: u64 = 0;
    for (i, &op) in ops.iter().enumerate() {
        let mut op_result: u64 = number_grid[0][i];
        for number_row in &number_grid[1..] {
            op_result = apply_operation(op, op_result, number_row[i]);
        }
        grand_total += op_result;
    }
    println!("{grand_total}");
}
