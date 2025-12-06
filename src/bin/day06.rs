use aoc25::read_lines_with_spaces;

// Day 6 - Took me a while to get part 2 right, I kept using read_lines which trimmed spaces
// (like an idiot)
// Output: Solution -> Part1: 4076006202939 || Part2: 7903168391557
fn main() {
    let input = include_str!("../../inputs/day06.txt");

    let arr: Vec<&str> = read_lines_with_spaces(input).collect();
    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i64, i64) {
    // Part 1: original row-wise
    let pg = get_problem_groups(input);
    let op = get_operators(input);
    let p1 = solve_worksheet(&pg, &op);

    // Part 2: column-wise, right-to-left
    let pg = get_problem_groups_part2(input);
    // The effective problem for p2 of the worksheet is the same as p1
    let p2 = solve_worksheet(&pg, &op);

    (p1, p2)
}

// Part 1
// Fairly easy solution, just get the problem groups (Vec<Vec<i64>>) and apply the operator to the
// group where the operator is
// Complexity: O(n), n = size of grid
fn solve_worksheet(problem: &Vec<Vec<i64>>, operator: &Vec<String>) -> i64 {
    let mut total: i64 = 0;

    for p in 0..problem.len() {
        let row = problem.get(p).unwrap();

        let val = match operator.get(p).map(|s| s.as_str()) {
            Some("+") => row.iter().sum::<i64>(),
            Some("-") => row.iter().copied().reduce(|acc, x| acc - x).unwrap(),
            Some("*") => row.iter().product::<i64>(),
            Some("/") => row.iter().copied().reduce(|acc, x| acc / x).unwrap(),
            _ => 0,
        };

        total += val;
    }
    total
}

// Find problem groups by reading the input lines left-to-right (p1 - ignoring spaces)
// Group comumn-wise for each
fn get_problem_groups(input: &[&str]) -> Vec<Vec<i64>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let re = regex::Regex::new(r"\d+").unwrap();
    for line in 0..input.len() {
        groups.push(
            re.find_iter(input[line])
                .map(|g| g.as_str().to_string())
                .collect(),
        );
    }

    let mut problem_groups: Vec<Vec<i64>> = vec![];
    for i in 0..groups[0].len() {
        let mut a: Vec<i64> = vec![];
        for j in 0..groups.len() {
            if let Some(val) = groups[j].get(i) {
                a.push(val.parse::<i64>().unwrap());
            }
        }
        problem_groups.push(a);
    }
    problem_groups
}

// Part 2
// This was slightly more tricky because spaces must be presrved for the int order
// Here we can use the space from one op to the next - 1 to work out the width of a group
// We can then treat this as a grid and keeping whitespaces get the column-wise groups as before
// Complexity: O(r*c), r = rows, c = cols -> O(n)
fn get_problem_groups_part2(input: &[&str]) -> Vec<Vec<i64>> {
    // Build a rectangular grid of chars using the last line (operator row) as the width.
    let op_line = input[input.len() - 1];
    let width = op_line.chars().count();

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(input.len());
    for line in input {
        let mut row: Vec<char> = line.chars().collect();
        if row.len() < width {
            row.extend(std::iter::repeat(' ').take(width - row.len()));
        } else if row.len() > width {
            row.truncate(width);
        }
        grid.push(row);
    }

    // Problems are separated by a full column of spaces
    let blocks = find_blocks(&grid);

    let rows = grid.len();
    let mut problems: Vec<Vec<i64>> = Vec::new();

    for (start, end) in blocks {
        let mut nums: Vec<i64> = Vec::new();

        // Each column within this block is one number (top digit at the top row)
        for c in start..=end {
            let mut digits = String::new();

            for r in 0..rows - 1 {
                let ch = grid[r][c];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }

            if !digits.is_empty() {
                let n: i64 = digits.parse().unwrap();
                nums.push(n);
            }
        }

        problems.push(nums);
    }

    problems
}

// Assuming operators will always be the last line
fn get_operators(input: &[&str]) -> Vec<String> {
    let re = regex::Regex::new(r"[+\-*/]").unwrap();
    let operators: Vec<String> = re
        .find_iter(input[input.len() - 1])
        .map(|g| g.as_str().to_string())
        .collect();
    operators
}

// Scan the worksheet column-wise and splits into problem groups
fn find_blocks(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;

    while c < cols {
        let is_sep = (0..rows).all(|r| grid[r][c] == ' ');
        if is_sep {
            c += 1;
            continue;
        }

        let start = c;
        c += 1;
        while c < cols {
            let is_sep = (0..rows).all(|r| grid[r][c] == ' ');
            if is_sep {
                break;
            }
            c += 1;
        }
        let end = c - 1;
        blocks.push((start, end));
    }

    blocks
}
