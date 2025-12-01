use std::io::BufRead;

fn read_lines(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path).expect("Error opening input");
    return std::io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error reading line"))
        .collect();
}

fn main() {
    let mut dial: u32 = 50;
    let mut zero_count: u32 = 0;

    for line in read_lines("day01/input") {
        let (direction_str, rotation_str) = line.split_at(1);
        let rotation = rotation_str.parse::<u32>().expect("Invalid rotation");

        match direction_str {
            "L" => {
                let mut inv_dial = (100 - dial) % 100;

                inv_dial += rotation;
                zero_count += inv_dial / 100;
                inv_dial = inv_dial % 100;

                dial = (100 - inv_dial) % 100;
            }
            "R" => {
                dial += rotation;
                zero_count += dial / 100;
                dial = dial % 100;
            }
            _ => panic!("Invalid direction {}", direction_str),
        }
    }

    println!("{}", zero_count);
}
