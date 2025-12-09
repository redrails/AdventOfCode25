use aoc25::read_lines;

// Day 8
// Output: Solution -> Part1: 80446 || Part2: 51294528
fn main() {
    let input = include_str!("../../inputs/day08.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i32, i32) {
    (0, 0)
}
