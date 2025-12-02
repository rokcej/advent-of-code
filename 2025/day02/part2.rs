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
    if id_str.len() < 2 {
        return true;
    }

    let max_chunk_size = id_str.len() / 2;
    for chunk_size in 1..=max_chunk_size {
        if id_str.len() % chunk_size != 0 {
            continue;
        }

        let mut chunks = id_str.as_bytes().chunks(chunk_size);
        let first_chunk = chunks.next().expect("Missing first chunk");
        if chunks.all(|chunk| chunk == first_chunk) {
            return false;
        }
    }

    return true;
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
