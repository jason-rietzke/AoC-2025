fn main() {
    let input = std::fs::read_to_string(format!("{}/data.txt", env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to read input file");
    let performace_start = std::time::Instant::now();
    let part1_result = solve_part1(&input, 1000);
    let performance_duration = performace_start.elapsed();
    println!("Part 1: {} (in {:?})", part1_result, performance_duration);

    let performace_start = std::time::Instant::now();
    let part2_result = solve_part2(&input);
    let performance_duration = performace_start.elapsed();
    println!("Part 2: {} (in {:?})", part2_result, performance_duration);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    index: usize,
    position: (usize, usize, usize),
    possible_links: Vec<Link>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Link {
    from: usize,
    to: usize,
    distance: u64,
}

/// find all distinct circuits in the graph (single nodes can be considered circuits of length 1)
fn find_circuits(nodes: &Vec<Node>, links: &Vec<Link>) -> Vec<Vec<usize>> {
    let mut visited = vec![false; nodes.len()];
    let mut circuits: Vec<Vec<usize>> = vec![];

    for node in nodes {
        if !visited[node.index] {
            let mut circuit: Vec<usize> = vec![];
            let mut stack: Vec<usize> = vec![node.index];
            while let Some(current_index) = stack.pop() {
                if !visited[current_index] {
                    visited[current_index] = true;
                    circuit.push(current_index);
                    let neighbors: Vec<usize> = links
                        .iter()
                        .filter_map(|link| {
                            if link.from == current_index {
                                Some(link.to)
                            } else if link.to == current_index {
                                Some(link.from)
                            } else {
                                None
                            }
                        })
                        .collect();
                    for neighbor in neighbors {
                        if !visited[neighbor] {
                            stack.push(neighbor);
                        }
                    }
                }
            }
            circuits.push(circuit);
        }
    }

    circuits
}

impl Node {
    fn from_str(index: usize, s: &str) -> Self {
        let coords: Vec<usize> = s
            .split(',')
            .map(|part| part.trim().parse::<usize>().unwrap())
            .collect();
        Node {
            index,
            position: (coords[0], coords[1], coords[2]),
            possible_links: vec![],
        }
    }

    /// Linear distance in 3D space with Euclidean metric
    fn square_distance(&self, other: &Node) -> u64 {
        let dx = (self.position.0 as isize - other.position.0 as isize).abs() as u64;
        let dy = (self.position.1 as isize - other.position.1 as isize).abs() as u64;
        let dz = (self.position.2 as isize - other.position.2 as isize).abs() as u64;
        dx * dx + dy * dy + dz * dz
    }

    fn evaluate_possible_links(&mut self, all_nodes: &Vec<Node>) {
        for node in all_nodes {
            if node.index != self.index {
                let distance = self.square_distance(node);
                self.possible_links.push(Link {
                    from: self.index,
                    to: node.index,
                    distance,
                });
            }
        }
    }
}

fn solve_part1(input: &str, shortest_connections_to_find: usize) -> u64 {
    let mut nodes: Vec<Node> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Node::from_str(index, line))
        .collect();

    for i in 0..nodes.len() {
        let mut node = nodes[i].clone();
        node.evaluate_possible_links(&nodes);
        nodes[i] = node;
    }

    let mut possible_links: Vec<Link> = nodes
        .iter()
        .flat_map(|node| node.possible_links.clone())
        .filter(|link| link.from < link.to)
        .collect();
    possible_links.sort_by_key(|link| link.distance);

    let links = possible_links
        .iter()
        .take(shortest_connections_to_find)
        .cloned()
        .collect();

    let circuits = find_circuits(&nodes, &links);
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let largest_circuit_sizes = circuit_sizes.iter().take(3);
    largest_circuit_sizes.fold(1, |acc, size| acc * (*size as u64))
}

fn solve_part2(input: &str) -> u64 {
    let mut nodes: Vec<Node> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Node::from_str(index, line))
        .collect();

    for i in 0..nodes.len() {
        let mut node = nodes[i].clone();
        node.evaluate_possible_links(&nodes);
        nodes[i] = node;
    }

    let mut possible_links: Vec<Link> = nodes
        .iter()
        .flat_map(|node| node.possible_links.clone())
        .filter(|link| link.from < link.to)
        .collect();
    possible_links.sort_by_key(|link| link.distance);
    let possible_links = possible_links;

    // binary search through all options until the lowest option that results in a single circuit is found
    let mut low = 1;
    let mut high = possible_links.len();
    while low < high {
        let mid = (low + high) / 2;
        let links: Vec<Link> = possible_links.iter().take(mid).cloned().collect();
        let circuits = find_circuits(&nodes, &links);
        if circuits.len() == 1 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let links: Vec<Link> = possible_links.iter().take(low).cloned().collect();

    let last_link = links.last().unwrap();
    nodes[last_link.from].position.0 as u64 * nodes[last_link.to].position.0 as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
    }

    #[test]
    fn test_part1() {
        let input = test_data();
        let expected_output = 40;
        assert_eq!(solve_part1(input, 10), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = test_data();
        let expected_output = 25272;
        assert_eq!(solve_part2(input), expected_output);
    }
}
