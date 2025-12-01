use aoc25::read_lines;

// Day 1: Secret Entrance
// Output: Solution -> Part1: 1191 || Part2: 6858

pub fn main() {
    let input = include_str!("../../inputs/day01.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2): (_, _) = (solve_part1(&arr), solve_part2(&arr));
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

// Part 1
fn solve_part1(input: &Vec<&str>) -> i32 {
    let mut result = 0;
    let mut pos = 50;

    // Simple solution of checking number of 0 "clicks" in total at each move
    for line in input {
        let amount: i32 = line[1..].parse().unwrap();
        pos = (pos + dir(line) * amount).rem_euclid(100);
        if pos == 0 {
            result += 1;
        }
    }

    result
}

// Part 2
fn solve_part2(input: &Vec<&str>) -> i32 {
    let mut result = 0;
    let mut pos: i32 = 50;

    // Not efficient but since the input isn't huge it's fine
    // Looping through each "click" and finding the amount of 0 hits
    // Still O(n)
    for line in input {
        let amount: i32 = line[1..].trim().parse().unwrap();
        for _ in 0..amount {
            pos = (pos + dir(line)).rem_euclid(100);

            if pos == 0 {
                result += 1;
            }
        }
    }

    result
}

// R = 1, L = -1
fn dir(line: &str) -> i32 {
    if line.chars().next().unwrap() == 'R' {
        1
    } else {
        -1
    }
}
