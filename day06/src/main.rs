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

enum Operation {
    Add,
    Multiply,
}
struct Exercise {
    values: Vec<u64>,
    operation: Operation,
}

fn solve_part1(input: &str) -> u64 {
    let mut exercises: Vec<Exercise> = vec![];
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        for (i, part) in parts.iter().enumerate() {
            if exercises.len() <= i {
                exercises.push(Exercise {
                    values: vec![],
                    operation: Operation::Add,
                });
            }
            let exercise = &mut exercises[i];
            match *part {
                "+" => exercise.operation = Operation::Add,
                "*" => exercise.operation = Operation::Multiply,
                num_str => {
                    let number = num_str.parse().expect("Failed to parse number");
                    exercise.values.push(number);
                }
            }
        }
    }

    exercises.iter().map(compute_exercise).sum()
}

fn solve_part2(input: &str) -> u64 {
    let mut exercises: Vec<Exercise> = Vec::new();
    exercises.push(Exercise {
        values: vec![],
        operation: Operation::Add,
    });
    let lines = input.lines().collect::<Vec<&str>>();
    for c in (0..lines[0].len()).rev() {
        let mut parts = vec![];
        for r in 0..lines.len() {
            let part = &lines[r][c..=c];
            parts.push(part.trim());
        }
        // create new exercise if a blank column is found
        if parts.iter().all(|p| p.is_empty()) {
            exercises.push(Exercise {
                values: vec![],
                operation: Operation::Add,
            });
            continue;
        }
        // read all (but the last line) parts as a number
        let num_str = parts[..parts.len() - 1]
            .iter()
            .filter(|p| !p.is_empty())
            .cloned()
            .collect::<Vec<&str>>()
            .join("");
        let num = num_str.parse().expect("Failed to parse number");
        let exercise = exercises.last_mut().expect("No exercise found");
        exercise.values.push(num);
        // read the last line as operation
        let op_str = parts[parts.len() - 1];
        match op_str {
            "+" => exercise.operation = Operation::Add,
            "*" => exercise.operation = Operation::Multiply,
            _ => {}
        }
    }

    exercises.iter().map(compute_exercise).sum()
}

fn compute_exercise(exercise: &Exercise) -> u64 {
    match exercise.operation {
        Operation::Add => exercise.values.iter().sum(),
        Operation::Multiply => exercise.values.iter().product(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let expected_output = 4277556;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let expected_output = 3263827;
        assert_eq!(solve_part2(input), expected_output);
    }
}
