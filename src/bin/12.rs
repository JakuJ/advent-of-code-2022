#![feature(test)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

extern crate bucket_queue;
use bucket_queue::*;

type Point = (i32, i32);

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Copy)]
struct Node {
    coords: Point,
    value: char,
}

#[derive(Debug)]
struct Graph {
    width: usize,
    vertices: Vec<Node>,
    edges: HashMap<Node, Vec<Node>>,
    start: Node,
    end: Node,
}

fn manhattan(a: Point, b: Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn parse_graph_from_grid_of_adjacent_chars(input: &str, reverse: bool) -> Graph {
    let mut nodes = Vec::new();
    let mut edges = HashMap::new();
    let mut start = None;
    let mut end = None;

    let width = input.lines().next().unwrap().len();

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
            if manhattan(node.coords, other_node.coords) == 1
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
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph_from_grid_of_adjacent_chars(input, false);

    // define heuristic as integer euclidan distance to end
    let heuristic = |node: Node| manhattan(node.coords, graph.end.coords);

    // find shortest path from start to end in graph using A* algorithm
    // https://en.wikipedia.org/wiki/A*_search_algorithm
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse((heuristic(graph.start), graph.start)));

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(graph.start, 0);
    f_score.insert(graph.start, heuristic(graph.start));

    while let Some(Reverse((_, current))) = open_set.pop() {
        if current == graph.end {
            let mut path = Vec::new();
            let mut current = current;
            while let Some(prev) = came_from.get(&current) {
                path.push(current);
                current = *prev;
            }
            path.push(current);
            path.reverse();
            return Some(path.len() - 1);
        }

        for neighbor in graph.edges.get(&current).unwrap() {
            let tentative_g_score = g_score[&current] + 1;
            if tentative_g_score < *g_score.get(neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                f_score.insert(*neighbor, tentative_g_score + heuristic(*neighbor));
                open_set.push(Reverse((f_score[neighbor], *neighbor)));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph = parse_graph_from_grid_of_adjacent_chars(input, true);

    // Dial's algorithm for finding paths from end to all other nodes
    let mut dist = vec![usize::MAX; graph.edges.len()];
    let mut queue = BucketQueue::<VecDeque<Node>>::new();

    let index = |node: &Node| (node.coords.0 + node.coords.1 * graph.width as i32) as usize;

    dist[index(&graph.end)] = 0;
    queue.enqueue(graph.end, 0);

    while let Some(current) = queue.dequeue_min() {
        let c_ix = index(&current);

        for neighbor in graph.edges.get(&current).unwrap() {
            let n_ix = index(neighbor);

            let dist_c = dist[c_ix];
            let dist_n = dist[n_ix];

            if dist_n > dist_c + 1 {
                dist[n_ix] = dist_c + 1;
                queue.enqueue(*neighbor, dist_c + 1);
            }
        }
    }

    Some(
        *dist
            .iter()
            .enumerate()
            .filter(|&(ix, _)| graph.vertices[ix].value == 'a')
            .map(|(_, b)| b)
            .min()
            .unwrap_or(&0),
    )
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
