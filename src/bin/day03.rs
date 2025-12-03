use aoc25::read_lines;

// Day 3
// Output: Solution -> Part1: 117524 || Part2: 173848577117276
fn main() {
    let input = include_str!("../../inputs/day03.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i32, i64) {
    let mut sum = 0;
    let mut sum2 = 0;
    for line in input {
        sum += find_joltage(line);
        sum2 += find_joltage_n(line);
    }
    (sum, sum2)
}

// Part 1
// Easy solution since only two batteries are required
// Find max in first set which is all but last character
// Find max in second set which is all characters after the first max
// Complexity: O(n) where n is the number of batteries
fn find_joltage(input: &str) -> i32 {
    let chars: Vec<char> = input.chars().collect();

    let first_digit = chars[..chars.len() - 1].iter().max().unwrap();
    let first_digit_idx = chars.iter().position(|&c| c == *first_digit).unwrap();

    let second_digit = chars[first_digit_idx + 1..].iter().max().unwrap();

    format!("{}{}", first_digit, second_digit).parse().unwrap()
}

// Part 2
// Slightly more diffcult solution since n batteries are required
// Loop with start = the last max found + 1, and rem = n - number of max found
// Keep doing this until no digits remain, in effect this is until the end of the string is reached
// Complexity: O(n*m) -> O(n) where n is the # of batteries and m is the # of digits to find
fn find_joltage_n(input: &str) -> i64 {
    let chars: Vec<char> = input.chars().collect();

    if chars.len() <= 12 {
        return input.parse().unwrap();
    }

    let mut result = String::new();
    let mut start = 0;
    let mut rem = 12;

    while rem > 0 {
        let end = chars.len() - rem;

        let mut max_dig = '0';
        let mut max_dig_idx = start;

        for i in start..=end {
            if chars[i] > max_dig {
                max_dig = chars[i];
                max_dig_idx = i;
            }
        }

        result.push(max_dig);
        start = max_dig_idx + 1;
        rem -= 1;
    }

    result.parse().unwrap()
}
