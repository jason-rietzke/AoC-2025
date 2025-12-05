use std::vec;

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

fn solve_part1(input: &str) -> u64 {
    let mut joltages = vec![];
    for line in input.lines() {
        let values = line
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        joltages.push(find_biggest_number(values, 2));
    }
    joltages.iter().sum()
}

fn solve_part2(input: &str) -> u64 {
    let mut joltages = vec![];
    for line in input.lines() {
        let values = line
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        joltages.push(find_biggest_number(values, 12));
    }
    joltages.iter().sum()
}

fn find_biggest_number(values: Vec<u64>, digit_amount: u8) -> u64 {
    let mut digits = vec![];
    let mut start_index = 0;
    for _ in 0..digit_amount {
        let mut largest_value_index = start_index;
        for i in start_index..values.len() - digit_amount as usize + 1 + digits.len() {
            if values[i] > values[largest_value_index] {
                largest_value_index = i;
            }
            if values[largest_value_index] == 9 {
                break;
            }
        }
        digits.push(values[largest_value_index]);
        start_index = largest_value_index + 1;
    }
    let digits_str = digits
        .iter()
        .map(|&d| d.to_string())
        .collect::<Vec<String>>()
        .join("");
    digits_str.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let expected_output = 357;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let expected_output = 3121910778619;
        assert_eq!(solve_part2(input), expected_output);
    }
}
