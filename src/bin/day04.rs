use aoc25::read_lines;

// Day 4
// Output: Solution -> Part1: 1491 || Part2: 8722
fn main() {
    let input = include_str!("../../inputs/day04.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i32, i32) {
    (find_rolls(input), find_rolls_mod(input))
}

// Relative directions of adjacent cells from existing (x,y) in the grid
// isize because of negative values, not i32/i64 otherwise explicit casting will be required later
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Part 1
// Easy solutuon, just look in all DIRECTIONS and find '@'
// If reached 4, break and sum
// Complexity: O(n^2) - The operation is O(n*m) where n = number of cols and m = number of rows,
// since every operation has DIRECTIION checking, the work per iteration results to O(n^2)
fn find_rolls(input: &[&str]) -> i32 {
    let mut sum = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i].chars().nth(j) != Some('@') {
                continue;
            }

            let mut found = 0;

            for (di, dj) in &DIRECTIONS {
                let adj_i = i as isize + *di;
                let adj_j = j as isize + *dj;

                if adj_i < 0 || adj_j < 0 {
                    continue;
                }

                if let Some(row) = input.get(adj_i as usize) {
                    if let Some(ch) = row.chars().nth(adj_j as usize) {
                        if ch == '@' {
                            found += 1;

                            if found >= 4 {
                                break;
                            }
                        }
                    }
                }
            }

            if found < 4 {
                sum += 1;
            }
        }
    }

    sum
}

// Part 2
// Slightly more difficult, the concept is the same but the grid will be modified each iteration to
// remove adjacent paper rolls and create space.
// Complexity: O(n^2) similar to Part 1 with work required, this time the space complexity is much
// higher due to mutable grid being stored.
fn find_rolls_mod(input: &[&str]) -> i32 {
    // Create grid from original input that is mutable so we can remove paper rolls
    let mut grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();

    let mut sum = 0;

    loop {
        // What can be removed in this round
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != '@' {
                    continue; // Only care about paper rolls
                }

                let mut found = 0;

                // Similar to p1, just checking surroundings
                for (di, dj) in &DIRECTIONS {
                    let adj_i = i as isize + *di;
                    let adj_j = j as isize + *dj;

                    if adj_i < 0 || adj_j < 0 {
                        continue;
                    }

                    let (ai, aj) = (adj_i as usize, adj_j as usize);

                    // Out of bounds
                    if ai >= grid.len() || aj >= grid[ai].len() {
                        continue;
                    }

                    // Found adjacent within this grid cycle
                    if grid[ai][aj] == '@' {
                        found += 1;

                        if found >= 4 {
                            break;
                        }
                    }
                }

                if found < 4 {
                    to_remove.push((i, j));
                }
            }
        }

        // Nothing can be removed at this point
        if to_remove.is_empty() {
            break;
        }

        // Apply the removal vec to the grid (mutable)
        for (i, j) in to_remove {
            if grid[i][j] == '@' {
                grid[i][j] = '.';
                sum += 1;
            }
        }
    }

    sum
}
