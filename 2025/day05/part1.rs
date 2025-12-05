fn main() {
    let input: String = std::fs::read_to_string("day05/input").expect("Error reading input");

    let mut lines = input.lines();
    let fresh_id_ranges: Vec<&str> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    let ids: Vec<&str> = lines.skip(1).collect();

    let fresh_id_ranges: Vec<(u64, u64)> = fresh_id_ranges
        .into_iter()
        .map(|range| {
            let (min, max) = range.split_once("-").expect("Invalid range");
            return (
                min.parse().expect("Invalid range min"),
                max.parse().expect("Invalid range max"),
            );
        })
        .collect();
    let ids: Vec<u64> = ids
        .into_iter()
        .map(|id| id.parse().expect("Invalid ID"))
        .collect();

    let mut fresh_id_count: u64 = 0;
    for id in ids {
        for &(range_min, range_max) in fresh_id_ranges.iter() {
            if id >= range_min && id <= range_max {
                fresh_id_count += 1;
                break;
            }
        }
    }
    println!("{fresh_id_count}");
}
