fn parse_position(line: &str) -> (u64, u64) {
    let (x, y) = line.split_once(',').unwrap();
    return (x.parse().unwrap(), y.parse().unwrap());
}

fn main() {
    let positions: Vec<(u64, u64)> = std::fs::read_to_string("day09/input")
        .expect("Error reading input")
        .lines()
        .map(parse_position)
        .collect();

    let mut max_area: u64 = 0;
    for (i, &(x, y)) in positions.iter().enumerate() {
        for &(x2, y2) in &positions[i + 1..] {
            let width = x.abs_diff(x2) + 1;
            let height = y.abs_diff(y2) + 1;
            max_area = max_area.max(width * height);
        }
    }
    println!("{max_area}");
}
