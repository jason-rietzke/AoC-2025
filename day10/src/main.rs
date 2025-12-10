use good_lp::*;

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

struct MachineSchematic {
    expected_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl MachineSchematic {
    fn from_str(input: &str) -> Self {
        let mut expected_lights = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        let parts: Vec<&str> = input.split(' ').collect();
        let lights_str = parts[0];
        for c in lights_str.chars().skip(1).take(lights_str.len() - 2) {
            expected_lights.push(c == '#');
        }
        for part in &parts[1..parts.len() - 1] {
            let button_str = part.trim_matches(|c| c == '(' || c == ')');
            let button_indices: Vec<usize> = button_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            buttons.push(button_indices);
        }
        let joltages_str = parts[parts.len() - 1];
        let joltages_cleaned = joltages_str.trim_matches(|c| c == '{' || c == '}');
        for joltage_part in joltages_cleaned.split(',') {
            let joltage = joltage_part.trim().parse::<u64>().unwrap();
            joltages.push(joltage);
        }

        MachineSchematic {
            expected_lights,
            buttons,
            joltages,
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    let schematics: Vec<MachineSchematic> = input
        .lines()
        .map(|line| MachineSchematic::from_str(line))
        .collect();
    let mut min_button_presses = vec![u64::MAX; schematics.len()];

    for (i, schematic) in schematics.iter().enumerate() {
        let num_buttons = schematic.buttons.len();
        let num_combinations = 1 << num_buttons;

        for combo in 0..num_combinations {
            let mut lights_state = vec![false; schematic.expected_lights.len()];
            let mut presses = 0;
            for button_index in 0..num_buttons {
                if presses >= min_button_presses[i] {
                    break;
                }
                if (combo & (1 << button_index)) != 0 {
                    presses += 1;
                    for &light_index in &schematic.buttons[button_index] {
                        lights_state[light_index] = !lights_state[light_index];
                    }
                }
            }
            if lights_state == schematic.expected_lights {
                min_button_presses[i] = presses;
            }
        }
    }

    min_button_presses.iter().sum()
}

fn solve_part2(input: &str) -> u64 {
    let schematics: Vec<MachineSchematic> = input
        .lines()
        .map(|line| MachineSchematic::from_str(line))
        .collect();

    let mut solutions = Vec::new();

    for schematic in &schematics {
        let expected_joltage = &schematic.joltages;
        let num_buttons = schematic.buttons.len();

        // uppress CBC output
        let _suppress = SuppressStdout::new();

        // integer variables for button presses
        let mut vars = variables!();
        let button_vars: Vec<Variable> = (0..num_buttons)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        // problem: minimize sum of button presses
        let mut problem = vars
            .minimise(button_vars.iter().sum::<Expression>())
            .using(default_solver);

        // constraints: for each joltage position, sum = target
        for (joltage_idx, &target) in expected_joltage.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (button_idx, button) in schematic.buttons.iter().enumerate() {
                if button.contains(&joltage_idx) {
                    expr = expr + button_vars[button_idx];
                }
            }
            problem = problem.with(constraint!(expr == target as i32));
        }

        match problem.solve() {
            Ok(solution) => {
                let presses: u64 = button_vars
                    .iter()
                    .map(|&var| solution.value(var) as u64)
                    .sum();
                solutions.push(presses);
            }
            Err(e) => {
                eprintln!("No solution found: {:?}", e);
            }
        }
    }

    solutions.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    }

    #[test]
    fn test_part1() {
        let input = test_data();
        let expected_output = 7;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = test_data();
        let expected_output = 33;
        assert_eq!(solve_part2(input), expected_output);
    }
}

// Suppress stdout for CBC solver output
struct SuppressStdout {
    saved_fd: i32,
}
impl SuppressStdout {
    fn new() -> Self {
        use std::io::Write;
        let _ = std::io::stdout().flush(); // Flush Rust's stdout buffer
        unsafe {
            let saved_fd = libc::dup(libc::STDOUT_FILENO);
            let devnull = libc::open(b"/dev/null\0".as_ptr() as *const i8, libc::O_WRONLY);
            libc::dup2(devnull, libc::STDOUT_FILENO);
            libc::close(devnull);
            Self { saved_fd }
        }
    }
}
impl Drop for SuppressStdout {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.saved_fd, libc::STDOUT_FILENO);
            libc::close(self.saved_fd);
        }
    }
}
