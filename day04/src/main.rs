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
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_rolls = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                if evaluate_accessibility(&grid, r, c) {
                    accessible_rolls += 1;
                }
            }
        }
    }

    accessible_rolls
}

fn solve_part2(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_roll_count = grid.iter().flatten().filter(|&&ch| ch == '@').count() as u64;
    let rows = grid.len();
    let cols = grid[0].len();

    let mut removed_rolls = vec![];
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                if evaluate_accessibility(&grid, r, c) {
                    grid[r][c] = '.';
                    removed_rolls.push((r, c));
                }
            }
        }
    }

    while !removed_rolls.is_empty() {
        let mut new_removed_rolls = vec![];
        for (r, c) in removed_rolls {
            let accessible_adjacent_rolls = evaluate_adjacent_rolls(&mut grid, r, c);
            for (ar, ac) in accessible_adjacent_rolls {
                grid[ar][ac] = '.';
                if !new_removed_rolls.contains(&(ar, ac)) {
                    new_removed_rolls.push((ar, ac));
                }
            }
        }
        removed_rolls = new_removed_rolls;
    }

    start_roll_count - grid.iter().flatten().filter(|&&ch| ch == '@').count() as u64
}

fn evaluate_accessibility(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut adjacent_rolls = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            // skip self
            if dr == 0 && dc == 0 {
                continue;
            }
            // handle edges
            if (dr < 0 && r == 0)
                || (dr > 0 && r == rows - 1)
                || (dc < 0 && c == 0)
                || (dc > 0 && c == cols - 1)
            {
                continue;
            }
            if grid[(r as isize + dr) as usize][(c as isize + dc) as usize] == '@' {
                adjacent_rolls += 1;
            }
        }
    }

    adjacent_rolls < 4
}

// Evaluate adjacent rolls and return a vec of all accessible rolls (their coordinates)
fn evaluate_adjacent_rolls(grid: &mut Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_rolls = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            // skip self
            if dr == 0 && dc == 0 {
                continue;
            }
            // handle edges
            if (dr < 0 && r == 0)
                || (dr > 0 && r == rows - 1)
                || (dc < 0 && c == 0)
                || (dc > 0 && c == cols - 1)
            {
                continue;
            }
            let nr = (r as isize + dr) as usize;
            let nc = (c as isize + dc) as usize;
            if grid[nr][nc] == '@' && evaluate_accessibility(grid, nr, nc) {
                accessible_rolls.push((nr, nc));
            }
        }
    }
    accessible_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected_output = 13;
        assert_eq!(solve_part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected_output = 43;
        assert_eq!(solve_part2(input), expected_output);
    }
}
