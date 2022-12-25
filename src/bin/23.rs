#![feature(test)]
#![feature(box_patterns)]
#![feature(box_syntax)]

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32);
type Map = HashSet<Point>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, -(y as i32)))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_map(input);

    let check_north = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let north = (-1..=1).all(|dx| !map.contains(&(x + dx, y + 1)));
        if north {
            match propositions.entry((x, y + 1)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_south = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let south = (-1..=1).all(|dx| !map.contains(&(x + dx, y - 1)));
        if south {
            match propositions.entry((x, y - 1)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_west = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let west = (-1..=1).all(|dy| !map.contains(&(x - 1, y + dy)));
        if west {
            match propositions.entry((x - 1, y)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_east = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let east = (-1..=1).all(|dy| !map.contains(&(x + 1, y + dy)));
        if east {
            match propositions.entry((x + 1, y)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let mut direction_checks: VecDeque<
        Box<dyn FnMut((i32, i32), &mut HashMap<Point, Option<Point>>, &Map) -> bool>,
    > = VecDeque::new();

    direction_checks.push_back(box check_north);
    direction_checks.push_back(box check_south);
    direction_checks.push_back(box check_west);
    direction_checks.push_back(box check_east);

    let mut propositions = HashMap::with_capacity(map.len());
    for _ in 0..10 {
        // first half of the round
        for &(x, y) in map.iter() {
            // if 8 cardinal directions empty, do nothing
            let mut alone = true;
            'search: for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if map.contains(&(x + dx, y + dy)) {
                        alone = false;
                        break 'search;
                    }
                }
            }

            if alone {
                continue;
            }

            // if not alone, propose a move
            for check in direction_checks.iter_mut() {
                if check((x, y), &mut propositions, &map) {
                    break;
                }
            }
        }

        // evaluate propositions
        for ((tx, ty), src) in propositions.iter() {
            if let Some((x, y)) = src {
                map.remove(&(*x, *y));
                map.insert((*tx, *ty));
            }
        }
        propositions.clear();

        // shift directions
        let fst = direction_checks.pop_front().unwrap();
        direction_checks.push_back(fst);
    }

    // find area of bounding rectangle of all points in the map
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for &(x, y) in map.iter() {
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

    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    Some(area as usize - map.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_map(input);

    let check_north = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let north = (-1..=1).all(|dx| !map.contains(&(x + dx, y + 1)));
        if north {
            match propositions.entry((x, y + 1)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_south = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let south = (-1..=1).all(|dx| !map.contains(&(x + dx, y - 1)));
        if south {
            match propositions.entry((x, y - 1)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_west = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let west = (-1..=1).all(|dy| !map.contains(&(x - 1, y + dy)));
        if west {
            match propositions.entry((x - 1, y)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let check_east = |(x, y), propositions: &mut HashMap<Point, Option<Point>>, map: &Map| {
        let east = (-1..=1).all(|dy| !map.contains(&(x + 1, y + dy)));
        if east {
            match propositions.entry((x + 1, y)) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }
            true
        } else {
            false
        }
    };

    let mut direction_checks: VecDeque<
        Box<dyn FnMut((i32, i32), &mut HashMap<Point, Option<Point>>, &Map) -> bool>,
    > = VecDeque::new();

    direction_checks.push_back(box check_north);
    direction_checks.push_back(box check_south);
    direction_checks.push_back(box check_west);
    direction_checks.push_back(box check_east);

    let mut propositions = HashMap::with_capacity(map.len());
    let mut turn = 1;
    loop {
        // first half of the round
        for &(x, y) in map.iter() {
            // if 8 cardinal directions empty, do nothing
            let mut alone = true;
            'search: for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if map.contains(&(x + dx, y + dy)) {
                        alone = false;
                        break 'search;
                    }
                }
            }

            if alone {
                continue;
            }

            // if not alone, propose a move
            for check in direction_checks.iter_mut() {
                if check((x, y), &mut propositions, &map) {
                    break;
                }
            }
        }

        // evaluate propositions
        let mut any_changed = false;
        for ((tx, ty), src) in propositions.iter() {
            if let Some((x, y)) = src {
                map.remove(&(*x, *y));
                map.insert((*tx, *ty));
                any_changed = true;
            }
        }

        if !any_changed {
            break Some(turn);
        }

        propositions.clear();

        // shift directions
        let fst = direction_checks.pop_front().unwrap();
        direction_checks.push_back(fst);

        turn += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 23);
        assert_eq!(part_one(&input), Some(4049));
        assert_eq!(part_two(&input), Some(1021));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 23);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 23);
        b.iter(|| part_two(input));
    }
}
