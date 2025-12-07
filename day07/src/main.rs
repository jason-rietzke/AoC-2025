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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldType {
    Space,
    Start,
    Splitter,
    SplitterActive,
    Beam(u64),
}

fn solve_part1(input: &str) -> u64 {
    let (mut grid, start_pos) = parse_data(input);
    propagate_beam(&mut grid, start_pos.0 + 1, start_pos.1);
    grid.iter()
        .flatten()
        .filter(|&&field| field == FieldType::SplitterActive)
        .count() as u64
}

fn solve_part2(input: &str) -> u64 {
    let (mut grid, start_pos) = parse_data(input);
    let permutations = propagate_beam(&mut grid, start_pos.0 + 1, start_pos.1);
    permutations
}

fn parse_data(input: &str) -> (Vec<Vec<FieldType>>, (usize, usize)) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mut grid = vec![vec![FieldType::Space; cols]; rows];
    let mut start_pos = (0, 0);
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = match ch {
                '.' => FieldType::Space,
                'S' => {
                    start_pos = (r, c);
                    FieldType::Start
                }
                '^' => FieldType::Splitter,
                '|' => FieldType::Beam(0),
                _ => panic!("Unknown character in input: {}", ch),
            };
        }
    }
    (grid, start_pos)
}

fn propagate_beam(grid: &mut Vec<Vec<FieldType>>, r: usize, c: usize) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();
    if r >= rows || c >= cols {
        return 1;
    }
    match grid[r][c] {
        FieldType::Start => panic!("Beam cannot spawn on the Start element"),
        FieldType::Beam(permutations) => permutations,
        FieldType::Space => {
            let permutations = propagate_beam(grid, r + 1, c);
            grid[r][c] = FieldType::Beam(permutations);
            permutations
        }
        FieldType::Splitter | FieldType::SplitterActive => {
            grid[r][c] = FieldType::SplitterActive;
            // left and right
            propagate_beam(grid, r, c - 1) + propagate_beam(grid, r, c + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected_output = 21;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected_output = 40;
        assert_eq!(solve_part2(input), expected_output);
    }
}
