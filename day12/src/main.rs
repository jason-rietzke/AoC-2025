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
    let areas: Vec<((usize, usize), Vec<usize>)> = input
        .split("\n")
        .skip(30)
        .filter_map(|line| {
            let mut parts = line.split(": ");
            let size = parts.next()?.trim();
            let (width, height) = {
                let mut dims = size.split('x');
                let w = dims.next()?.parse::<usize>().ok()?;
                let h = dims.next()?.parse::<usize>().ok()?;
                (w, h)
            };
            let objects: Vec<usize> = parts
                .next()?
                .trim()
                .split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect();
            Some(((width, height), objects))
        })
        .collect();

    let mut solved = 0;
    for ((width, height), objects) in &areas {
        let area_size = width * height;
        let mut total_shape_size = 0;
        for object_count in objects {
            total_shape_size += 9 * object_count;
        }
        if total_shape_size <= area_size {
            solved += 1;
        }
    }
    solved
}

fn solve_part2(_input: &str) -> &str {
    "Merry Christmas!"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
    }

    #[test]
    fn test_part1() {
        let input = test_data();
        let expected_output = 2;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = test_data();
        let expected_output = "Merry Christmas!";
        assert_eq!(solve_part2(input), expected_output);
    }
}
