use aoc25::read_lines;

// Day 8
// Output: Solution -> Part1: 80446 || Part2: ?
// Entry point: reads input and prints both part results.
fn main() {
    let input = include_str!("../../inputs/day09.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i64, i64) {
    (largest_rectangle_p1(input), largest_rectangle_p2(input))
}

// Part 1 - Easy solution
// For peformance subsequent grid points beyond x can be ignored
// O(n^2) n = grid size
fn largest_rectangle_p1(input: &[&str]) -> i64 {
    let red_tiles: Vec<(i64, i64)> = parse_input_as_pairs(input);
    let mut largest_area: i64 = 0;
    for i in 0..red_tiles.len() {
        // optimisation: don't need to repeat area calcs
        for j in (i + 1)..red_tiles.len() {
            let area = get_rect_area(red_tiles[i], red_tiles[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

// Part 2 - a lot more difficult than p1 and doesn't currently work
// Computes the largest valid rectangle area
// Complexity: O(n^2) n = grid size - Probably, who knows?
fn largest_rectangle_p2(input: &[&str]) -> i64 {
    let red_tiles: Vec<(i64, i64)> = parse_input_as_pairs(input);
    if red_tiles.len() < 2 {
        return 0;
    }

    let (min_x, _max_x, min_y, _max_y) = get_bounds(&red_tiles);

    let grid = build_grid_of_greens_p2(input);
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let to_grid =
        |x: i64, y: i64| -> (usize, usize) { ((x - min_x) as usize, (y - min_y) as usize) };

    let mut largest_area: i64 = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let min_rx = x1.min(x2);
            let max_rx = x1.max(x2);
            let min_ry = y1.min(y2);
            let max_ry = y1.max(y2);

            let mut ok = true;
            'outer: for y in min_ry..=max_ry {
                for x in min_rx..=max_rx {
                    let (gx, gy) = to_grid(x, y);
                    if gx >= width || gy >= height || grid[gy][gx] == '.' {
                        ok = false;
                        break 'outer;
                    }
                }
            }

            if ok {
                let area = get_rect_area((x1, y1), (x2, y2));
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    largest_area
}

// Builds a grid marking red tiles and all green tiles for p2
fn build_grid_of_greens_p2(input: &[&str]) -> Vec<Vec<char>> {
    let red_tiles: Vec<(i64, i64)> = parse_input_as_pairs(input);

    // Get bounds to draw greens, this is just min/max(x) and min/max(y)
    let (min_x, max_x, min_y, max_y) = get_bounds(&red_tiles);

    // Build grid
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid = vec![vec!['.'; width]; height];

    draw_boundaries_p2(&red_tiles, min_x, min_y, &mut grid);

    let mut stack = Vec::new();

    for x in 0..width {
        if grid[0][x] == '.' {
            stack.push((x, 0));
        }
        if grid[height - 1][x] == '.' {
            stack.push((x, height - 1));
        }
    }

    for y in 0..height {
        if grid[y][0] == '.' {
            stack.push((0, y));
        }
        if grid[y][width - 1] == '.' {
            stack.push((width - 1, y));
        }
    }

    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height {
            continue;
        }
        if grid[y][x] != '.' {
            continue;
        }
        grid[y][x] = 'O';

        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < width {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < height {
            stack.push((x, y + 1));
        }
    }

    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                'O' => grid[y][x] = '.',
                '.' => grid[y][x] = 'X',
                _ => {}
            }
        }
    }

    grid
}

// Draws the red seq and its boundary greens onto the grid
fn draw_boundaries_p2(red_tiles: &[(i64, i64)], min_x: i64, min_y: i64, grid: &mut Vec<Vec<char>>) {
    let to_grid = |x: i64, y: i64| -> (usize, usize) {
        let gx = (x - min_x) as usize;
        let gy = (y - min_y) as usize;
        (gx, gy)
    };

    // Mark the red tiles
    for &(x, y) in red_tiles {
        let (gx, gy) = to_grid(x, y);
        grid[gy][gx] = '#';
    }

    // Draw lines between red tiles
    for i in 0..(red_tiles.len() - 1) {
        draw_segment(red_tiles[i], red_tiles[i + 1], grid, &to_grid);
    }

    draw_segment(red_tiles[red_tiles.len() - 1], red_tiles[0], grid, &to_grid);
}

// Gets the boundary of the grid points: (min_x, max_x, min_y, max_y)
fn get_bounds(points: &[(i64, i64)]) -> (i64, i64, i64, i64) {
    let mut min_x = points[0].0;
    let mut max_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    for &(x, y) in points {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    (min_x, max_x, min_y, max_y)
}

// Draws a straight segment of boundary greens between two red tiles
fn draw_segment(
    a: (i64, i64),
    b: (i64, i64),
    grid: &mut Vec<Vec<char>>,
    to_grid: &dyn Fn(i64, i64) -> (usize, usize),
) {
    let (x1, y1) = a;
    let (x2, y2) = b;

    if y1 == y2 {
        let (sx, ex) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        for x in (sx + 1)..ex {
            let (gx, gy) = to_grid(x, y1);
            if grid[gy][gx] == '.' {
                grid[gy][gx] = 'X';
            }
        }
    } else {
        let (sy, ey) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in (sy + 1)..ey {
            let (gx, gy) = to_grid(x1, y);
            if grid[gy][gx] == '.' {
                grid[gy][gx] = 'X';
            }
        }
    }
}

// Calculates the area of a rectangle defined by two grid points (inclusive)
fn get_rect_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let width = (x1.max(x2) - x1.min(x2)) + 1;
    let height = (y1.max(y2) - y1.min(y2)) + 1;

    width * height
}

// Parses the input lines into a list of (x, y) coordinate pairs
fn parse_input_as_pairs(input: &[&str]) -> Vec<(i64, i64)> {
    input
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<_> = line.split(',').collect();
            (
                parts[0].trim().parse::<i64>().unwrap(),
                parts[1].trim().parse::<i64>().unwrap(),
            )
        })
        .collect()
}
