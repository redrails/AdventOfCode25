use aoc25::read_lines;

// Day 8
// Output: Solution -> Part1: 558 || Part2: ?
// Entry point: reads input and prints both part results.
fn main() {
    let input = include_str!("../../inputs/day10.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i64, i64) {
    let p1 = get_button_presses_p1(input);
    let p2 = get_button_presses_p2(input);
    (p1, p2)
}

// Part 1
// Easy solution using brute force to try all button combination and look for the least
// Complexity: O(2^n) where n = number of button combinations - very inefficient

fn get_button_presses_p1(input: &[&str]) -> i64 {
    let mut sum = 0;

    for line in input {
        let (indicator, buttons, _joltage) = parse_line(line);

        // "[.##.]" -> ".##."
        let pattern = indicator.trim_matches(|c| c == '[' || c == ']');

        let mut target: Vec<bool> = Vec::new();
        for ch in pattern.chars() {
            target.push(ch == '#');
        }

        let light_count = target.len();
        let button_count = buttons.len();

        // Each button is pressed 0 or 1 time
        let mut presses_per_button = vec![0i64; button_count];
        let mut best = i64::MAX;
        let mut done = false;

        while !done {
            // Start with all lights off
            let mut lights = vec![false; light_count];

            // Apply button presses
            for b in 0..button_count {
                if presses_per_button[b] == 1 {
                    for &idx in &buttons[b] {
                        lights[idx] = !lights[idx];
                    }
                }
            }

            if lights == target {
                let presses: i64 = presses_per_button.iter().sum();
                if presses < best {
                    best = presses;
                }
            }

            let mut pos = 0;
            loop {
                if pos == button_count {
                    done = true;
                    break;
                }

                if presses_per_button[pos] == 0 {
                    presses_per_button[pos] = 1;
                    break;
                } else {
                    presses_per_button[pos] = 0;
                    pos += 1;
                }
            }
        }

        sum += best;
    }

    sum
}

// Part 2
// A lot more difficult, I tried to use the same brute force technique as p1
// Fails to run (tried for 15 mins -> i7 13900K, 64GB ram, maybe I should run on GPU ^^)
// Complexity: Probably something like O(n^2 + 2^n) or something
fn get_button_presses_p2(input: &[&str]) -> i64 {
    let mut total_presses = 0;

    for line in input {
        let (_indicator, buttons, target) = parse_line(line);

        let counter_count = target.len();
        let button_count = buttons.len();

        // Precompute total increments needed across all counters
        // (sum of final joltage values)
        let total_target_increments: i64 = target.iter().sum();

        // For each button, simple upper bound: smallest target it affects
        let mut button_max: Vec<i64> = Vec::new();
        for b in 0..button_count {
            let affected = &buttons[b];
            let mut m = i64::MAX;
            for &idx in affected {
                let t = target[idx];
                if t < m {
                    m = t;
                }
            }
            button_max.push(m);
        }

        let mut presses_per_button: Vec<i64> = vec![0; button_count];
        let mut best: i64 = i64::MAX;
        let mut done = false;

        while !done {
            // Compute total increments this combination would produce
            let total_increments_here: i64 = presses_per_button
                .iter()
                .enumerate()
                .map(|(b, &times)| times * buttons[b].len() as i64)
                .sum();

            // If the total increments don't match the total needed,
            // we can skip this combo entirely
            if total_increments_here == total_target_increments {
                // Simulate this combination
                let mut counters = vec![0i64; counter_count];

                for b in 0..button_count {
                    let times = presses_per_button[b];
                    if times == 0 {
                        continue;
                    }
                    for _ in 0..times {
                        for &idx in &buttons[b] {
                            counters[idx] += 1;
                        }
                    }
                }

                if counters == target {
                    let presses_here: i64 = presses_per_button.iter().sum();
                    if presses_here < best {
                        best = presses_here;
                    }
                }
            }

            let mut pos = 0;
            loop {
                if pos == button_count {
                    done = true;
                    break;
                }

                if presses_per_button[pos] < button_max[pos] {
                    presses_per_button[pos] += 1;
                    break;
                } else {
                    presses_per_button[pos] = 0;
                    pos += 1;
                }
            }
        }

        total_presses += best;
    }

    total_presses
}

// Helper to parse the line in the input easily for p1 and p2
fn parse_line(line: &str) -> (String, Vec<Vec<usize>>, Vec<i64>) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // First part: indicator pattern, e.g. "[.##.]"
    let indicator = parts[0].to_string();

    // Last part: joltage requirements, e.g. "{3,5,4,7}"
    let joltage_str = parts[parts.len() - 1];
    let joltage_inner = joltage_str.trim_matches(|c| c == '{' || c == '}');
    let joltage: Vec<i64> = joltage_inner
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Middle parts: buttons, e.g. "(3)", "(1,3)", ...
    let button_parts = &parts[1..parts.len() - 1];
    let buttons: Vec<Vec<usize>> = button_parts
        .iter()
        .map(|s| {
            s.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (indicator, buttons, joltage)
}
