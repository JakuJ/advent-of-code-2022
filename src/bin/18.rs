#![feature(test)]

use advent_of_code::helpers::parse_with_regex;
use lazy_static::lazy_static;
use regex::Regex;

type Point = (usize, usize, usize);

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
}

// parse CSV lines into points
fn parse_line(line: &str) -> Point {
    let [x, y, z] = parse_with_regex::<usize, 3>(&RE, line);
    (x, y, z)
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = input.lines().map(parse_line).collect::<Vec<_>>();

    // find max dimensions
    let max_x = points.iter().map(|(x, _, _)| x).max().unwrap() + 2;
    let max_y = points.iter().map(|(_, y, _)| y).max().unwrap() + 2;
    let max_z = points.iter().map(|(_, _, z)| z).max().unwrap() + 2;

    // for each cube side, find the number of cubes that contain it
    let mut sides = vec![vec![vec![vec![0; 3]; max_z]; max_y]; max_x];

    for (x, y, z) in points {
        // floor
        sides[x][y][z][0] += 1;
        // ceiling
        sides[x][y][z + 1][0] += 1;
        // front
        sides[x][y][z][1] += 1;
        // back
        sides[x][y + 1][z][1] += 1;
        // left side
        sides[x][y][z][2] += 1;
        // right side
        sides[x + 1][y][z][2] += 1;
    }

    Some(
        sides
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|&&x| x == 1)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let points = input
        .lines()
        .map(parse_line)
        .map(|(x, y, z)| (x + 1, y + 1, z + 1))
        .collect::<Vec<_>>();

    // find max dimensions
    let max_x = points.iter().map(|(x, _, _)| x).max().unwrap() + 2;
    let max_y = points.iter().map(|(_, y, _)| y).max().unwrap() + 2;
    let max_z = points.iter().map(|(_, _, z)| z).max().unwrap() + 2;

    // for each cube side, find the number of cubes that contain it
    let mut sides = vec![vec![vec![vec![0; 3]; max_z]; max_y]; max_x];

    for (x, y, z) in points {
        // floor
        sides[x][y][z][0] += 1;
        // ceiling
        sides[x][y][z + 1][0] += 1;
        // front
        sides[x][y][z][1] += 1;
        // back
        sides[x][y + 1][z][1] += 1;
        // left side
        sides[x][y][z][2] += 1;
        // right side
        sides[x + 1][y][z][2] += 1;
    }

    // flood fill
    let mut queue = vec![(0, 0, 0)];
    let mut visited = vec![vec![vec![false; max_z]; max_y]; max_x];
    let mut reached_sides = vec![vec![vec![vec![false; 3]; max_z]; max_y]; max_x];

    while let Some((x, y, z)) = queue.pop() {
        visited[x][y][z] = true;

        // floor
        if sides[x][y][z][0] == 1 {
            reached_sides[x][y][z][0] = true;
        } else if z > 0 && !visited[x][y][z - 1] {
            // go down
            queue.push((x, y, z - 1));
        }
        // ceiling
        if z + 1 < max_z && sides[x][y][z + 1][0] == 1 {
            reached_sides[x][y][z + 1][0] = true;
        } else if z + 1 < max_z && !visited[x][y][z + 1] {
            // go up
            queue.push((x, y, z + 1));
        }
        // front
        if sides[x][y][z][1] == 1 {
            reached_sides[x][y][z][1] = true;
        } else if y > 0 && !visited[x][y - 1][z] {
            // go back
            queue.push((x, y - 1, z));
        }
        // back
        if y + 1 < max_y && sides[x][y + 1][z][1] == 1 {
            reached_sides[x][y + 1][z][1] = true;
        } else if y + 1 < max_y && !visited[x][y + 1][z] {
            // go forward
            queue.push((x, y + 1, z));
        }
        // left
        if sides[x][y][z][2] == 1 {
            reached_sides[x][y][z][2] = true;
        } else if x > 0 && !visited[x - 1][y][z] {
            // go left
            queue.push((x - 1, y, z));
        }
        // right
        if x + 1 < max_x && sides[x + 1][y][z][2] == 1 {
            reached_sides[x + 1][y][z][2] = true;
        } else if x + 1 < max_x && !visited[x + 1][y][z] {
            // go right
            queue.push((x + 1, y, z));
        }
    }

    Some(
        reached_sides
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .map(|&side| side as u32)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 18);
        assert_eq!(part_one(&input), Some(3326));
        assert_eq!(part_two(&input), Some(1996));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 18);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 18);
        b.iter(|| part_two(input));
    }
}
