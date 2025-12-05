use aoc25::read_lines;

// Day 5
// Output: Solution -> Part1: 613 || Part2: 336495597913098
fn main() {
    let input = include_str!("../../inputs/day05.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i32, i64) {
    (find_fresh_part1(&input), find_ranges_part2(&input))
}

// Part 1
// Easy solution, extract the ranges (s, e) from all the lines above blank
// Loop through each range and check if the ingredients are fresh and sum
// Complexity: O(n*m), n = ranges, m = ingredients
fn find_fresh_part1(input: &[&str]) -> i32 {
    let ranges = get_ranges(&input);
    let mut sum = 0;

    for i in get_split_point(&input) + 1..input.len() {
        if in_range(&ranges, input[i].parse::<i64>().unwrap()) {
            sum += 1;
        }
    }

    sum
}

// Part 2 - Not so easy
// Only get the ranges as we don't care about the ingredients
// Sort the pairs (s, e) on both
// Check overlap by comparing s, e of each range and sum if no overlap
// Otherwise do an inclusive sum if there is an overlap
// Complexity: `sort_unstable_by` is O(n log n) worstcase, then O(n) on the check
fn find_ranges_part2(input: &[&str]) -> i64 {
    let ranges = get_ranges(input);

    if ranges.is_empty() {
        return 0;
    }

    // Reassign ranges so we can mutate (sort)
    let mut ranges = ranges.to_vec();

    // Sort by start, then by end
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let (mut cur_start, mut cur_end) = ranges[0];
    let mut total: i64 = 0;

    for &(s, e) in &ranges[1..] {
        if s <= cur_end {
            // Overlapping – extend current interval if needed
            if e > cur_end {
                cur_end = e;
            }
        } else {
            // No overlap – close off previous interval
            total += cur_end - cur_start + 1; // inclusive length
            cur_start = s;
            cur_end = e;
        }
    }

    // Final interval
    total += cur_end - cur_start + 1;

    total
}

// Takes the list of the first part of the input and returns a list of (s, e)
fn get_ranges(input: &[&str]) -> Vec<(i64, i64)> {
    let mut ranges = vec![];
    for i in 0..get_split_point(&input) {
        let pair = input[i].split('-').collect::<Vec<&str>>();
        ranges.push((
            pair[0].parse::<i64>().unwrap(),
            pair[1].parse::<i64>().unwrap(),
        ));
    }
    ranges
}

// Finds the split point and returns its index
fn get_split_point(input: &[&str]) -> usize {
    input.iter().position(|&x| x.is_empty()).unwrap()
}

// Checks whether or not an value is in any of the ranges in the first part
fn in_range(ranges: &[(i64, i64)], value: i64) -> bool {
    for &(start, end) in ranges {
        if value as i64 >= start && value as i64 <= end {
            return true;
        }
    }
    false
}
