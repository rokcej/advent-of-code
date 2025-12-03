fn byte_to_digit(byte: u8) -> u8 {
    assert!(byte >= b'0' && byte <= b'9');
    return byte - b'0';
}

fn find_largest_digit(text: &str) -> (usize, u8) {
    let mut max_digit: u8 = byte_to_digit(text.bytes().next().expect("Empty text"));
    let mut max_index: usize = 0;

    for (index, byte) in text.bytes().enumerate().skip(1) {
        if max_digit >= 9 {
            break;
        }
        let digit = byte_to_digit(byte);
        if digit > max_digit {
            max_digit = digit;
            max_index = index;
        }
    }

    return (max_index, max_digit);
}

fn main() {
    let input: String = std::fs::read_to_string("day03/input").expect("Error reading input");
    const BATTERY_COUNT: usize = 12;
    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        let mut joltage: u64 = 0;
        let mut i_start: usize = 0;

        for i_battery in 0..BATTERY_COUNT {
            let i_end = line.len() - (BATTERY_COUNT - i_battery);
            let (i_digit, digit) = find_largest_digit(&line[i_start..=i_end]);
            i_start += i_digit + 1;
            joltage = 10 * joltage + (digit as u64);
        }

        total_joltage += joltage;
    }

    println!("{}", total_joltage);
}
