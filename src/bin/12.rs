#![feature(test)]

extern crate bucket_queue;
use bucket_queue::*;
use std::collections::HashMap;

type Point = (i32, i32);

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    coords: Point,
    value: char,
}

impl Node {
    fn distance(&self, other: &Node) -> u32 {
        ((self.coords.0 - other.coords.0).abs() + (self.coords.1 - other.coords.1).abs()) as u32
    }
}

struct Graph {
    width: usize,
    vertices: Vec<Node>,
    edges: HashMap<Node, Vec<Node>>,
    start: Node,
    end: Node,
}

impl Graph {
    pub fn index(&self, node: &Node) -> usize {
        (node.coords.0 + node.coords.1 * self.width as i32) as usize
    }
}

fn parse_graph_from_grid_of_adjacent_chars(input: &str, reverse: bool) -> Graph {
    let mut nodes = Vec::new();
    let mut edges = HashMap::new();
    let mut start = None;
    let mut end = None;

    let width = input.lines().next().expect("Input cannot be empty").len();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut node = Node {
                coords: (x as i32, y as i32),
                value: c,
            };
            if node.value == 'S' {
                node = Node {
                    coords: node.coords,
                    value: 'a',
                };
                start = Some(node);
            } else if node.value == 'E' {
                node = Node {
                    coords: node.coords,
                    value: 'z',
                };
                end = Some(node);
            }
            nodes.push(node);
        }
    }

    for node in &nodes {
        let mut adjacent_nodes = Vec::new();
        for other_node in &nodes {
            if node.distance(other_node) == 1
                && (if reverse {
                    node.value as i32 - other_node.value as i32 <= 1
                } else {
                    other_node.value as i32 - node.value as i32 <= 1
                })
            {
                adjacent_nodes.push(*other_node);
            }
        }
        edges.insert(*node, adjacent_nodes);
    }

    Graph {
        vertices: nodes,
        width,
        edges,
        start: start.expect("No start node found"),
        end: end.expect("No end node found"),
    }
}

/// A* algorithm for finding shortest path from Start node to End node.
pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph_from_grid_of_adjacent_chars(input, false);

    let heuristic = |node: Node| node.distance(&graph.end) as usize;

    // find shortest path from start to end in graph using A* algorithm
    // https://en.wikipedia.org/wiki/A*_search_algorithm
    let mut open_set = BucketQueue::<Vec<Node>>::new();

    open_set.push(graph.start, heuristic(graph.start));

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(graph.start, 0);
    f_score.insert(graph.start, heuristic(graph.start));

    while let Some(current) = open_set.pop_min() {
        if current == graph.end {
            let mut path = Vec::new();
            let mut current = current;
            while let Some(prev) = came_from.get(&current) {
                path.push(current);
                current = *prev;
            }
            return Some(path.len());
        }

        for neighbor in &graph.edges[&current] {
            let tentative_g_score = g_score[&current] + 1_usize;
            if tentative_g_score < *g_score.get(neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                f_score.insert(*neighbor, tentative_g_score + heuristic(*neighbor));
                open_set.push(*neighbor, f_score[neighbor]);
            }
        }
    }

    None
}

/// Dial's algorithm for finding paths from the End node to all other nodes.
/// Complexity: O(E + V), but since E = 4V, this is O(V)
pub fn part_two(input: &str) -> Option<usize> {
    let graph = parse_graph_from_grid_of_adjacent_chars(input, true);

    let mut dist = vec![usize::MAX; graph.vertices.len()];
    let mut queue = BucketQueue::<Vec<Node>>::new();

    dist[graph.index(&graph.end)] = 0;
    queue.push(graph.end, 0);

    while let Some(current) = queue.pop_min() {
        let dist_c_p1 = dist[graph.index(&current)] + 1;

        for neighbor in &graph.edges[&current] {
            let n_ix = graph.index(neighbor);

            if dist[n_ix] > dist_c_p1 {
                dist[n_ix] = dist_c_p1;
                queue.push(*neighbor, dist_c_p1);
            }
        }
    }

    dist.iter()
        .enumerate()
        .filter(|&(ix, _)| graph.vertices[ix].value == 'a')
        .map(|(_, &d)| d)
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 12);
        assert_eq!(part_one(&input), Some(481));
        assert_eq!(part_two(&input), Some(480));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 12);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 12);
        b.iter(|| part_two(input));
    }
}
