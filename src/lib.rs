// Read input lines into Interator, trimming whitespace
pub fn read_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|line| line.trim())
}
