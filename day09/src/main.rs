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
    let positions = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|num| num.parse::<u64>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();
    let mut largest_area: u64 = 0;

    for i in 0..positions.len() {
        let (x1, y1) = positions[i];
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            let (x2, y2) = positions[j];
            let left = x1.min(x2);
            let right = x1.max(x2);
            let top = y1.min(y2);
            let bottom = y1.max(y2);
            let area = (right - left + 1) * (bottom - top + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn solve_part2(input: &str) -> u64 {
    let positions = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|num| num.parse::<u64>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();
    let mut largest_area: u64 = 0;

    for i in 0..positions.len() {
        let (x1, y1) = positions[i];
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            let (x2, y2) = positions[j];
            let left = x1.min(x2);
            let right = x1.max(x2);
            let top = y1.min(y2);
            let bottom = y1.max(y2);

            // early return if area is smaller than the largest found so far
            let area = (right - left + 1) * (bottom - top + 1);
            if area < largest_area {
                continue;
            }

            // check that in this rectangle no lines (from adjacent points) cross the area
            let mut invalid = false;
            for k in 0..positions.len() {
                let first_point = positions[k];
                let second_point = positions[(k + 1) % positions.len()];
                let line_left = first_point.0.min(second_point.0);
                let line_right = first_point.0.max(second_point.0);
                let line_top = first_point.1.min(second_point.1);
                let line_bottom = first_point.1.max(second_point.1);

                if line_left < right && line_right > left && line_top < bottom && line_bottom > top
                {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                continue;
            }
            largest_area = area;
        }
    }

    largest_area
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
    }

    #[test]
    fn test_part1() {
        let input = test_data();
        let expected_output = 50;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = test_data();
        let expected_output = 24;
        assert_eq!(solve_part2(input), expected_output);
    }
}
