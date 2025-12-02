use std::io::BufRead;

fn read_lines(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path).expect("Error opening input");
    return std::io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error reading line"))
        .collect();
}

fn is_id_valid(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 == 1 {
        return true;
    }

    let (first_half, second_half) = id_str.split_at(id_str.len() / 2);
    return first_half != second_half;
}

fn main() {
    let mut invalid_id_sum: u64 = 0;

    for line in read_lines("day02/input") {
        for range in line.split(',') {
            let (first_id, last_id) = range.split_once('-').expect("Invalid range");
            let first_id: u64 = first_id.parse().expect("Invalid range first");
            let last_id: u64 = last_id.parse().expect("Invalid range last");
            for id in first_id..=last_id {
                if !is_id_valid(id) {
                    invalid_id_sum += id;
                }
            }
        }
    }

    println!("{}", invalid_id_sum);
}
