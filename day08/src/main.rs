fn main() {
    let input = std::fs::read_to_string(format!("{}/data.txt", env!("CARGO_MANIFEST_DIR")))
        .expect("Failed to read input file");
    let performace_start = std::time::Instant::now();
    let (part1_result, part1_timing) = solve_part1(&input, 1000);
    let performance_duration = performace_start.elapsed();
    println!("Part 1: {} (in {:?}){}", part1_result, performance_duration, part1_timing);

    let performace_start = std::time::Instant::now();
    let (part2_result, part2_timing) = solve_part2(&input);
    let performance_duration = performace_start.elapsed();
    println!("Part 2: {} (in {:?}){}", part2_result, performance_duration, part2_timing);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    nodes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    index: usize,
    position: (usize, usize, usize),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Link {
    from: usize,
    to: usize,
    distance: u64,
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
        }
    }

    /// Linear distance in 3D space with Euclidean metric
    fn square_distance(&self, other: &Node) -> u64 {
        let dx = (self.position.0 as isize - other.position.0 as isize).abs() as u64;
        let dy = (self.position.1 as isize - other.position.1 as isize).abs() as u64;
        let dz = (self.position.2 as isize - other.position.2 as isize).abs() as u64;
        dx * dx + dy * dy + dz * dz
    }
}

fn calculate_links(nodes: &Vec<Node>, only_best: Option<usize>) -> Vec<Link> {
    let mut links = match only_best {
        Some(n) => Vec::with_capacity(n),
        None => Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2),
    };
    // (distance, link_index) sorted by distance
    let mut link_distances: Vec<(u64, usize)> = Vec::with_capacity(links.capacity());

    for node in nodes {
        for other_node in nodes {
            // avoid duplicate links and self-links
            if node.index < other_node.index {
                let distance = node.square_distance(other_node);
                match only_best {
                    Some(n) => {
                        if links.len() < n {
                            links.push(Link {
                                from: node.index,
                                to: other_node.index,
                                distance,
                            });
                            link_distances.push((distance, links.len() - 1));
                            link_distances.sort_by_key(|(dist, _)| *dist);
                        } else {
                            if distance < link_distances[n - 1].0 {
                                let (_, link_index) = link_distances.pop().unwrap();
                                links[link_index] = Link {
                                    from: node.index,
                                    to: other_node.index,
                                    distance,
                                };
                                link_distances.push((distance, link_index));
                                link_distances.sort_by_key(|(dist, _)| *dist);
                            }
                        }
                    }
                    None => {
                        links.push(Link {
                            from: node.index,
                            to: other_node.index,
                            distance,
                        });
                    }
                }
            }
        }
    }
    links.sort_by_key(|link| link.distance);
    links
}

/// calculating all distinct circuits with the given links
/// early returning if all nodes are in a single circuit
/// every node starts in its own circuit
fn calculate_circuits(nodes: &Vec<Node>, links: &Vec<Link>) -> (Vec<Circuit>, Vec<usize>) {
    let mut circuits: Vec<Circuit> = nodes
        .iter()
        .map(|node| Circuit {
            nodes: vec![node.index],
        })
        .collect();
    let mut links_in_use: Vec<usize> = vec![];

    for (link_index, link) in links.iter().enumerate() {
        // find the circuits that the link's nodes are in
        let mut circuit_indices: Vec<usize> = vec![];
        for (circuit_index, circuit) in circuits.iter().enumerate() {
            if circuit.nodes.contains(&link.from) || circuit.nodes.contains(&link.to) {
                circuit_indices.push(circuit_index);
            }
        }
        // if the link connects two different circuits, merge them
        if circuit_indices.len() == 2 {
            let first_circuit_index = circuit_indices[0];
            let second_circuit_index = circuit_indices[1];
            let mut first_circuit = circuits[first_circuit_index].clone();
            let second_circuit = circuits[second_circuit_index].clone();
            first_circuit
                .nodes
                .extend(second_circuit.nodes.iter().cloned());
            // remove the second circuit
            circuits.remove(second_circuit_index);
            // update the first circuit
            circuits[first_circuit_index] = first_circuit;
            links_in_use.push(link_index);
        }
        // early return if all nodes are in a single circuit
        if circuits.len() == 1 {
            break;
        }
    }

    (circuits, links_in_use)
}

fn solve_part1(input: &str, shortest_connections_to_find: usize) -> (u64, String) {
    let timer = std::time::Instant::now();
    let nodes: Vec<Node> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Node::from_str(index, line))
        .collect();
    let parse_time = std::time::Instant::now();

    let links = calculate_links(&nodes, Some(shortest_connections_to_find));
    let calc_time = std::time::Instant::now();

    let (circuits, _) = calculate_circuits(&nodes, &links);
    let circuit_time = std::time::Instant::now();

    let mut circuit_sizes: Vec<usize> =
        circuits.iter().map(|circuit| circuit.nodes.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let largest_circuit_sizes = circuit_sizes.iter().take(3);
    let result = largest_circuit_sizes.fold(1, |acc, size| acc * (*size as u64));
    let evaluation_time = std::time::Instant::now();

    let timing_info = format!(
        "\n\tParsing: {:?}\n\tCalculating links: {:?}\n\tCalculating circuits: {:?}\n\tEvaluating result: {:?}",
        parse_time.duration_since(timer),
        calc_time.duration_since(parse_time),
        circuit_time.duration_since(calc_time),
        evaluation_time.duration_since(circuit_time),
    );

    (result, timing_info)
}

fn solve_part2(input: &str) -> (u64, String) {
    let timer = std::time::Instant::now();
    let nodes: Vec<Node> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Node::from_str(index, line))
        .collect();
    let parse_time = std::time::Instant::now();

    let links = calculate_links(&nodes, None);
    let calc_time = std::time::Instant::now();

    let (_, links_in_use) = calculate_circuits(&nodes, &links);
    let circuit_time = std::time::Instant::now();

    let last_link = &links[links_in_use[links_in_use.len() - 1]];
    let result = nodes[last_link.from].position.0 as u64 * nodes[last_link.to].position.0 as u64;
    let evaluation_time = std::time::Instant::now();

    let timing_info = format!(
        "\n\tParsing: {:?}\n\tCalculating links: {:?}\n\tCalculating circuits: {:?}\n\tEvaluating result: {:?}",
        parse_time.duration_since(timer),
        calc_time.duration_since(parse_time),
        circuit_time.duration_since(calc_time),
        evaluation_time.duration_since(circuit_time),
    );

    (result, timing_info)
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
        assert_eq!(solve_part1(input, 10).0, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = test_data();
        let expected_output = 25272;
        assert_eq!(solve_part2(input).0, expected_output);
    }
}
