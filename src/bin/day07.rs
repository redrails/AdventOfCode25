use aoc25::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

// Day 5 - I found this to be a difficult one!
// Output: Solution -> Part1: 1667 || Part2: 62943905501815
fn main() {
    let input = include_str!("../../inputs/day07.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i64, i64) {
    (
        read_tachyons_part1(&input),
        read_quantum_tachyons_part2(&input),
    )
}

// Part 1
// This was a bit difficult because of the merging of beams
// Turns the input into a grid and does BFS over beam positions
// Complexity: O(n) we only process each cell in the grid once
fn read_tachyons_part1(input: &[&str]) -> i64 {
    let (grid, (sr, sc)) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let h = grid.len() as i32;
    let w = grid[0].len() as i32; // Assuming grid width is fixed

    // BFS over beam positions, merging overlapping beams via visited set
    let mut queue = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut sum: i64 = 0;

    queue.push_back((sr, sc));
    visited.insert((sr, sc));

    while let Some((r, c)) = queue.pop_front() {
        let nr = r + 1;
        if nr >= h {
            // Beam exits the manifold
            continue;
        }

        let ci = c as usize;
        let nri = nr as usize;
        let cell = grid[nri][ci];

        match cell {
            '.' | 'S' => {
                // Beam just continues straight down, no split here
                if visited.insert((nr, c)) {
                    queue.push_back((nr, c));
                }
            }
            '^' => {
                // Beam hits a splitter and it stops, and we create left/right beams
                sum += 1;

                // Left beam from (nr, c - 1)
                if c > 0 {
                    let lc = c - 1;
                    if visited.insert((nr, lc)) {
                        queue.push_back((nr, lc));
                    }
                }

                // Right beam from (nr, c + 1)
                if c + 1 < w {
                    let rc = c + 1;
                    if visited.insert((nr, rc)) {
                        queue.push_back((nr, rc));
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    sum
}

// Part 2
// Found this to be quite tricky because it's not number of beams alone anymore but timelines
// Keep propegating timelines row by row until they exceed the grid
// Total number of timelines that exit anywhere is the result
// Complexity: O(n) we only process each cell in the grid once
fn read_quantum_tachyons_part2(input: &[&str]) -> i64 {
    let (grid, (sr, sc)) = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    // Map of current beam positions -> number of timelines in that position
    let mut current: HashMap<(i32, i32), i64> = HashMap::new();
    current.insert((sr, sc), 1);

    let mut sum: i64 = 0;

    while !current.is_empty() {
        let mut next: HashMap<(i32, i32), i64> = HashMap::new();

        for (&(r, c), &count) in current.iter() {
            let nr = r + 1;
            if nr >= h {
                // All these timelines exit the manifold here
                sum += count;
                continue;
            }

            let ci = c as usize;
            let nri = nr as usize;
            let cell = grid[nri][ci];

            match cell {
                '.' | 'S' => {
                    // All timelines continue straight down
                    *next.entry((nr, c)).or_insert(0) += count;
                }
                '^' => {
                    // Split L R for the timeline
                    // Left branch
                    if c > 0 {
                        let lc = c - 1;
                        *next.entry((nr, lc)).or_insert(0) += count;
                    } else {
                        // Left would fall outside
                        sum += count;
                    }

                    // Right branch
                    if c + 1 < w {
                        let rc = c + 1;
                        *next.entry((nr, rc)).or_insert(0) += count;
                    } else {
                        // Right would fall outside
                        sum += count;
                    }
                }
                _ => {
                    // End
                    sum += count;
                }
            }
        }

        current = next;
    }

    sum
}

// Parse the input as grid to make it easier to find beam and splitter positions
// Returns the grid and the starting point (S), assuming S is always on the first row
fn parse_grid(input: &[&str]) -> (Vec<Vec<char>>, (i32, i32)) {
    let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();

    let start_col = grid[0]
        .iter()
        .position(|&ch| ch == 'S')
        .expect("S not found in first row") as i32;

    (grid, (0, start_col))
}
