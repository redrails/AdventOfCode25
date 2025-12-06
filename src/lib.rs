// Read input lines into Interator, trimming whitespace
pub fn read_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|line| line.trim())
}

// Read input lines into Interator, preserving spaces
pub fn read_lines_with_spaces(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

// Read input into vec split by comma
pub fn read_csv(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|part| {
            let parts: Vec<_> = part.split('-').collect();
            (
                parts[0].trim().parse::<i64>().unwrap(),
                parts[1].trim().parse::<i64>().unwrap(),
            )
        })
        .collect()
}
