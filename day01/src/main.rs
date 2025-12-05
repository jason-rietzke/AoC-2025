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

fn solve_part1(input: &str) -> i32 {
    let mut current = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value: i32 = value.parse().unwrap();
        (current, _) = perform_rotation(current, dir, value);
        if current == 0 {
            zero_counter += 1;
        }
    }
    zero_counter
}

fn solve_part2(input: &str) -> i32 {
    let mut current = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value: i32 = value.parse().unwrap();
        let zero_visits;
        (current, zero_visits) = perform_rotation(current, dir, value);
        zero_counter += zero_visits;
    }
    zero_counter
}

fn perform_rotation(current: i32, dir: &str, value: i32) -> (i32, i32) {
    let min = 0;
    let max = 99;
    let mut new_position = current;
    let mut zero_visits = 0;
    match dir {
        "L" => {
            for _ in 0..(value) {
                new_position -= 1;
                if new_position == 0 {
                    zero_visits += 1;
                }
                if new_position < min {
                    new_position = max;
                }
            }
        }
        "R" => {
            for _ in 0..(value) {
                new_position += 1;
                if new_position > max {
                    new_position = min;
                }
                if new_position == 0 {
                    zero_visits += 1;
                }
            }
        }
        _ => panic!("Invalid direction"),
    }
    (new_position, zero_visits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let expected_output = 3;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let expected_output = 6;
        assert_eq!(solve_part2(input), expected_output);
    }
}
