use aoc25::{parse_input_as_nums, read_lines};

// Day 8
// Output: Solution -> Part1: 80446 || Part2: 51294528
fn main() {
    let input = include_str!("../../inputs/day08.txt");

    let arr: Vec<&str> = read_lines(input).collect();

    let (p1, p2) = solve(&arr);
    println!("Solution -> Part1: {} || Part2: {}", p1, p2);
}

fn solve(input: &[&str]) -> (i32, i32) {
    // Parse to Vec<Vec<i64>>
    let points = parse_input_as_nums(input);
    let mut edges = build_edges(&points);
    // Sort the edges as they
    sort_edges_by_distance(&mut edges);

    let part1 = find_circuits_part1(&edges, points.len());
    let part2 = closest_unconnected_pairs_part2(&edges, &points);

    (part1, part2)
}

// Part 1
// Product of three largest circuits after 1000 connections
// Complexity: O(n^2) where n = number of junction boxes
fn find_circuits_part1(edges: &[(f64, usize, usize)], num_points: usize) -> i32 {
    let parent = connect_closest(edges, num_points, 1000);
    let mut sizes = circuit_sizes(parent);
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // biggest first

    (sizes[0] * sizes[1] * sizes[2]) as i32
}

// Part 2
// Last connection that makes everything one circuit finds product of x coordinates of that pair
// Complexity: O(n^2) where n = number of junction boxes
fn closest_unconnected_pairs_part2(edges: &[(f64, usize, usize)], points: &[Vec<i64>]) -> i32 {
    let n = points.len();

    // Start with each box in its own circuit.
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    // Number of separate circuits.
    let mut components = n;

    for &(_, a, b) in edges {
        // Find roots before deciding if this edge actually merges two circuits
        let root_a = find_root(&mut parent, a);
        let root_b = find_root(&mut parent, b);

        if root_a == root_b {
            // Already in the same circuit, don't do anything
            continue;
        }

        // This edge connects two different circuits
        union(&mut parent, &mut size, a, b);
        components -= 1;

        // When components becomes 1, this was the last connection
        if components == 1 {
            let x1 = points[a][0];
            let x2 = points[b][0];
            let product = x1 * x2;
            return product as i32;
        }
    }
    0
}

// Build (distance, i, j) for all pairs using euclidean_distance
fn build_edges(points: &[Vec<i64>]) -> Vec<(f64, usize, usize)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n.saturating_sub(1)) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = euclidean_distance(&points[i], &points[j]);
            edges.push((d, i, j));
        }
    }
    edges
}

// Sort edges by ascending distance
fn sort_edges_by_distance(edges: &mut Vec<(f64, usize, usize)>) {
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
}

// Connect the first K shortest edges using union-find logic
fn connect_closest(edges: &[(f64, usize, usize)], n: usize, k: usize) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let limit = k.min(edges.len());
    for idx in 0..limit {
        let (_, a, b) = edges[idx];
        union(&mut parent, &mut size, a, b);
    }
    parent
}

// Union two sets by size
fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) {
    let mut ra = find_root(parent, a);
    let mut rb = find_root(parent, b);
    if ra == rb {
        return;
    }

    if size[ra] < size[rb] {
        std::mem::swap(&mut ra, &mut rb);
    }

    parent[rb] = ra;
    size[ra] += size[rb];
}

// Find root iteratively
fn find_root(parent: &mut [usize], start: usize) -> usize {
    // First, walk up to find the root
    let mut root = start;
    while parent[root] != root {
        root = parent[root];
    }

    // Point everything along the way directly to root, walking up the tree
    let mut node = start;
    while parent[node] != node {
        let next = parent[node];
        parent[node] = root;
        node = next;
    }

    root
}

// Count sizes of each circuit (connected)
fn circuit_sizes(mut parent: Vec<usize>) -> Vec<usize> {
    let n = parent.len();
    let mut counts = vec![0; n];

    for i in 0..n {
        let r = find_root(&mut parent, i);
        counts[r] += 1;
    }
    counts.into_iter().filter(|&c| c > 0).collect()
}

// Calculate Euclidean distance between two points
fn euclidean_distance(a: &Vec<i64>, b: &Vec<i64>) -> f64 {
    assert!(a.len() == 3 && b.len() == 3);

    let dx = (a[0] - b[0]) as f64;
    let dy = (a[1] - b[1]) as f64;
    let dz = (a[2] - b[2]) as f64;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}
