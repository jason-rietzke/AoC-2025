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
    let ranges;
    let mut ingredients;
    (ranges, ingredients) = parse_data(input);

    for ingredient in &mut ingredients {
        for &(start, end) in &ranges {
            if ingredient.0 >= start && ingredient.0 <= end {
                ingredient.1 = true;
                break;
            }
        }
    }

    ingredients.iter().filter(|&&(_, fresh)| fresh).count() as u64
}

/// merging the ranges allows to have more compact representation that are also distinct from each other
/// distinct ranges are easily processed to calculate the total size since they do not overlap anymore
fn solve_part2(input: &str) -> u64 {
    let ranges;
    (ranges, _) = parse_data(input);

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|&(start, _)| start);
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for sorted_range in sorted_ranges {
        if let Some(last_merged_range) = merged_ranges.last_mut() {
            if sorted_range.0 <= last_merged_range.1 + 1 {
                last_merged_range.1 = last_merged_range.1.max(sorted_range.1);
            } else {
                merged_ranges.push(sorted_range);
            }
        } else {
            merged_ranges.push(sorted_range);
        }
    }

    let mut fresh_ingredients = 0;
    for range in merged_ranges {
        let range_size = range.1 - range.0 + 1;
        fresh_ingredients += range_size;
    }

    fresh_ingredients
}

fn parse_data(input: &str) -> (Vec<(u64, u64)>, Vec<(u64, bool)>) {
    let ranges: Vec<(u64, u64)> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect::<Vec<(u64, u64)>>();
    for range in &ranges {
        if range.0 > range.1 {
            panic!("Invalid range: {:?} - start is greater than end", range);
        }
    }

    let ingredients: Vec<(u64, bool)> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|line| line.parse::<u64>().ok())
        .map(|value| (value, false))
        .collect();

    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected_output = 3;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected_output = 14;
        assert_eq!(solve_part2(input), expected_output);
    }
}
