use aoc25::read_csv;

// Day 2
// Output: Solution -> Part1: 18700015741 || Part2: 20077272987
fn main() {
    let input = include_str!("../../inputs/day02.txt");

    let arr: Vec<(i64, i64)> = read_csv(input);

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

// Complexity: O(n*m) -> O(n) where n is the number of pairs and m is the size of the digits in the pairs
fn solve(input: &[(i64, i64)]) -> (i64, i64) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for &(x, y) in input {
        let mut i = x;
        while i < y + 1 {
            let is_invalid_id_1 = is_invalid_id_p1(i);
            if is_invalid_id_1 {
                sum1 += i
            }
            let is_invalid_id_2 = is_invalid_id_p2(i);
            if is_invalid_id_2 {
                sum2 += i
            }
            i += 1;
        }
    }
    (sum1, sum2)
}

// Part 1
// Split the number into two halves and check if they are the same
fn is_invalid_id_p1(id: i64) -> bool {
    let s = &id.to_string();
    let (a, b) = s.split_at(s.chars().count() / 2);
    a == b
}

// Part 2
// Repeat the number twice, chop off the first and last character if it contains the original number, then it's invalid because there is a repeated pattern.
fn is_invalid_id_p2(id: i64) -> bool {
    let s = &id.to_string();
    let rep = String::from(s).repeat(2);
    let chopped = &rep[1..rep.len() - 1];

    chopped.contains(s)
}
