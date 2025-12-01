use aoc25::read_lines;

// Day 1: Secret Entrance

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

    for line in input {
        let amount: i32 = line[1..].parse().unwrap();

        if line.starts_with("R") {
            // + operator
            pos = (pos + amount).rem_euclid(100);
        } else {
            // - operator
            pos = (pos - amount).rem_euclid(100);
        }

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

    for line in input {
        let dir = line.chars().next().unwrap();
        let amount: i32 = line[1..].trim().parse().unwrap();
        let step = if dir == 'R' { 1 } else { -1 };

        for _ in 0..amount {
            pos = (pos + step).rem_euclid(100);

            if pos == 0 {
                result += 1;
            }
        }
    }

    result
}
