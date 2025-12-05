fn can_merge_ranges(a: &(u64, u64), b: &(u64, u64)) -> bool {
    let overlap_min = a.0.max(b.0);
    let overlap_max = a.1.min(b.1);
    return overlap_min <= overlap_max;
}

fn get_merged_ranges(a: &(u64, u64), b: &(u64, u64)) -> (u64, u64) {
    return (a.0.min(b.0), a.1.max(b.1));
}

fn main() {
    let input: String = std::fs::read_to_string("day05/input").expect("Error reading input");

    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|range| {
            let (min, max) = range.split_once("-").expect("Invalid range");
            return (
                min.parse().expect("Invalid range min"),
                max.parse().expect("Invalid range max"),
            );
        })
        .collect();

    for i_dst in (0..ranges.len() - 1).rev() {
        for i_src in (i_dst + 1..ranges.len()).rev() {
            if can_merge_ranges(&ranges[i_dst], &ranges[i_src]) {
                ranges[i_dst] = get_merged_ranges(&ranges[i_dst], &ranges[i_src]);
                ranges.remove(i_src);
            }
        }
    }

    let range_sum: u64 = ranges.into_iter().map(|(min, max)| max + 1 - min).sum();
    println!("{range_sum}");
}
