struct Node {
    position: (u64, u64, u64),
    network_id: usize,
}

struct Pair {
    i: usize,
    j: usize,
    dist_sq: u64,
}

fn parse_position(line: &str) -> (u64, u64, u64) {
    let mut it = line
        .split(',')
        .map(|value| value.parse().expect("Invalid position"));
    return (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
}

fn get_distance_squared(a: &(u64, u64, u64), b: &(u64, u64, u64)) -> u64 {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    let dz = a.2.abs_diff(b.2);
    return dx * dx + dy * dy + dz * dz;
}
fn main() {
    let positions: Vec<(u64, u64, u64)> = std::fs::read_to_string("day08/input")
        .expect("Error reading input")
        .lines()
        .map(parse_position)
        .collect();

    let mut nodes: Vec<Node> = positions
        .into_iter()
        .enumerate()
        .map(|(i, position)| Node {
            position,
            network_id: i, // Each node initially belongs to its own network
        })
        .collect();

    let mut pairs: Vec<Pair> = Vec::with_capacity(nodes.len() * nodes.len().saturating_sub(1) / 2);
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let dist_sq = get_distance_squared(&nodes[i].position, &nodes[j].position);
            pairs.push(Pair { i, j, dist_sq });
        }
    }
    pairs.sort_by_key(|d| d.dist_sq);

    for pair in &pairs[..1000] {
        let new_network_id = nodes[pair.i].network_id;
        let old_network_id = nodes[pair.j].network_id;

        for node in &mut nodes {
            if node.network_id == old_network_id {
                node.network_id = new_network_id;
            }
        }
    }

    let mut network_sizes: Vec<u64> = vec![0; nodes.len()];
    for node in &nodes {
        network_sizes[node.network_id] += 1;
    }

    let mut enumerated_network_sizes: Vec<(usize, &u64)> =
        network_sizes.iter().enumerate().collect();
    enumerated_network_sizes.sort_by_key(|(_, size)| *size);

    let mut result: u64 = 1;
    for (_, size) in enumerated_network_sizes.iter().rev().take(3) {
        result *= *size;
    }
    println!("{result}");
}
