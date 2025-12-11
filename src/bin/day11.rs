use aoc25::read_lines;
use std::collections::HashMap;

// Day 11
// Output: Solution -> Part1: 571 || Part2: 511378159390560
fn main() {
    let input = include_str!("../../inputs/day11.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

const TARGET: &str = "out";

fn solve(input: &[&str]) -> (i64, i64) {
    let map = parse_input_to_map(input);

    (count_you_out_p1(&map), count_with_required_p2(&map))
}

// Part 1 - Easy solution
// Recursively check each path whilst storing the visited to avoid revisiting
// Essentially a DFS algorithm to search
// Complexity: O(V + E), standard DFS
fn count_you_out_p1(map: &HashMap<String, Vec<String>>) -> i64 {
    let start = "you";
    let mut visited: Vec<String> = Vec::new();

    count_paths(&map, start, TARGET, &mut visited)
}

// Part 2 - Slightly more difficult solution
// First time with normal DFS, got an infinite recursion because of too many node cycles
// Easy to solve by just using a cache (mem) which stores the visited paths and checks at each iter
// Mem stores the node and whether the required nodes are visited at this sequence, i.e: {node [seen_dac, seen_fft], X)}
// Complexity: O(V + E), standard DFS, possibly better because of the cache so not big recursions?
fn count_with_required_p2(map: &HashMap<String, Vec<String>>) -> i64 {
    let start = "svr";
    let required = vec!["dac", "fft"];
    let mut seen = vec![false; required.len()];
    let mut mem: HashMap<(String, Vec<bool>), i64> = HashMap::new();

    count_paths_with_required_p2(
        &map,
        start,
        TARGET,
        &required,
        &mut Vec::new(),
        &mut seen,
        &mut mem,
    )
}

fn count_paths(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut Vec<String>,
) -> i64 {
    // If we are at the target, we have found one complete path
    if current == target {
        return 1;
    }

    // Mark this node as part of the current path
    visited.push(current.to_string());

    let mut sum: i64 = 0;

    // Look at all neighbours of the current node
    if let Some(neighbours) = map.get(current) {
        for next in neighbours {
            let next = next.as_str();

            // Avoid revisiting nodes already in the current path
            let in_path = visited.iter().any(|s| s == next);
            if !in_path {
                sum += count_paths(map, next, target, visited);
            }
        }
    }

    visited.pop();
    sum
}

// Recursive function for p2 to count paths with required nodes
fn count_paths_with_required_p2(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    required: &[&str],
    visited: &mut Vec<String>,
    seen: &mut Vec<bool>,
    mem: &mut HashMap<(String, Vec<bool>), i64>,
) -> i64 {
    // First, update which required nodes we've seen at this node
    for (i, req) in required.iter().enumerate() {
        if current == *req {
            seen[i] = true;
        }
    }

    let key = (current.to_string(), seen.clone());

    // If we've already computed this state, just return it, from mem
    if let Some(&cached) = mem.get(&key) {
        return cached;
    }

    // If we reached the target, only count the path if all required nodes were seen
    if current == target {
        let ok = seen.iter().all(|v| *v);
        let result = if ok { 1 } else { 0 };
        mem.insert(key, result);
        return result;
    }

    visited.push(current.to_string());
    let mut sum = 0;

    if let Some(neighbours) = map.get(current) {
        for next in neighbours {
            let next_str = next.as_str();

            // Avoid revisiting nodes already on the current path
            let in_path = visited.iter().any(|s| s == next_str);
            if !in_path {
                // Clone the seen state for independent state management
                let mut child_seen = seen.clone();

                sum += count_paths_with_required_p2(
                    map,
                    next_str,
                    target,
                    required,
                    visited,
                    &mut child_seen,
                    mem,
                );
            }
        }
    }

    visited.pop();

    // Store the result for this (node, required visited?) to memory, this is very important
    mem.insert(key, sum);

    sum
}

// Helper function to parse input like: {node: [linked_nodes...]}
fn parse_input_to_map(input: &[&str]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in input {
        let (key, val) = line.split_once(": ").unwrap();

        let values = val
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        map.insert(key.to_string(), values);
    }
    map
}
