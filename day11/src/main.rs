use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string(format!("{}/data.txt", env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to read input file");
    let performace_start = std::time::Instant::now();
    let part1_result = solve_part1(&input);
    let performance_duration = performace_start.elapsed();
    println!("Part 1: {} (in {:?})", part1_result, performance_duration);

    let performace_start = std::time::Instant::now();
    let part2_result = solve_part2(&input);
    let performance_duration = performace_start.elapsed();
    println!("Part 2: {} (in {:?})", part2_result, performance_duration);
}

fn count_possible_paths(
    node: u64,
    out: u64,
    nodes: &HashMap<u64, Vec<u64>>,
    paths_cache: &mut HashMap<u64, u64>,
) -> u64 {
    if let Some(&cached) = paths_cache.get(&node) {
        return cached;
    }
    if node == out {
        return 1;
    }
    let mut total_paths = 0;
    if let Some(edges) = nodes.get(&node) {
        for edge in edges {
            let paths = count_possible_paths(*edge, out, nodes, paths_cache);
            total_paths += paths;
        }
    }
    paths_cache.insert(node, total_paths);
    total_paths
}

fn quick_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in s.as_bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(*byte as u64);
    }
    hash
}

fn solve_part1(input: &str) -> u64 {
    let nodes: HashMap<u64, Vec<u64>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let node = quick_hash(parts[0]);
            let edges = parts[1].split_whitespace().map(|s| quick_hash(s)).collect();
            (node, edges)
        })
        .collect();
    count_possible_paths(
        quick_hash("you"),
        quick_hash("out"),
        &nodes,
        &mut HashMap::<u64, u64>::new(),
    )
}

fn solve_part2(input: &str) -> u64 {
    let nodes: HashMap<u64, Vec<u64>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let node = quick_hash(parts[0]);
            let edges = parts[1].split_whitespace().map(|s| quick_hash(s)).collect();
            (node, edges)
        })
        .collect();
    let svr_to_fft = count_possible_paths(
        quick_hash("svr"),
        quick_hash("fft"),
        &nodes,
        &mut HashMap::<u64, u64>::new(),
    );
    let fft_to_dac = count_possible_paths(
        quick_hash("fft"),
        quick_hash("dac"),
        &nodes,
        &mut HashMap::<u64, u64>::new(),
    );
    let dac_to_out = count_possible_paths(
        quick_hash("dac"),
        quick_hash("out"),
        &nodes,
        &mut HashMap::<u64, u64>::new(),
    );

    svr_to_fft * fft_to_dac * dac_to_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let expected_output = 5;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let expected_output = 2;
        assert_eq!(solve_part2(input), expected_output);
    }
}
