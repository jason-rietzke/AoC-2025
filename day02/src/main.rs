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

fn solve_part1(input: &str) -> i64 {
    let mut invalid_ids = vec![];
    for ranges in input.trim().split(',') {
        let parts = ranges.split('-').collect::<Vec<&str>>();
        let first_id: i64 = parts[0].parse().unwrap();
        let last_id: i64 = parts[1].parse().unwrap();
        for id in first_id..=last_id {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            let mid = id_str.len() / 2;
            let (first_half, second_half) = id_str.split_at(mid);
            if first_half != second_half {
                continue;
            }
            if !invalid_ids.contains(&id) {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids.iter().sum()
}

fn solve_part2(input: &str) -> i64 {
    let mut invalid_ids = vec![];
    for ranges in input.trim().split(',') {
        let parts = ranges.split('-').collect::<Vec<&str>>();
        let first_id: i64 = parts[0].parse().unwrap();
        let last_id: i64 = parts[1].parse().unwrap();
        for id in first_id..=last_id {
            let id_str = id.to_string();
            let len = id_str.len();
            // calculate all possible divisors larger than 1
            let mut divisors = vec![];
            for i in 2..=len {
                if len % i == 0 {
                    divisors.push(i);
                }
            }
            // check if the id_str is composed of repeated substrings of length part_len
            for &divisor in &divisors {
                let part_len = len / divisor;
                let first_part = &id_str[0..part_len];
                let mut all_equal = true;
                for i in 1..divisor {
                    let start = i * part_len;
                    let end = start + part_len;
                    if &id_str[start..end] != first_part {
                        all_equal = false;
                        break;
                    }
                }
                if all_equal {
                    if !invalid_ids.contains(&id) {
                        invalid_ids.push(id);
                    }
                    break;
                }
            }
        }
    }
    invalid_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected_output = 1227775554;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected_output = 4174379265;
        assert_eq!(solve_part2(input), expected_output);
    }
}
