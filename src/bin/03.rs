#![feature(iter_array_chunks)]

fn priority(item: char) -> usize {
    let x = item as usize;
    if x <= 90 {
        27 + x - 65
    } else {
        x - 96
    }
}

type Set = [bool; 53];

fn set_of(rucksack: &str) -> Set {
    let mut set = [false; 53];
    for c in rucksack.chars() {
        set[priority(c)] = true;
    }
    set
}

fn set_intersects<const N: usize>(s: [&Set; N]) -> usize {
    for i in 1..=52 {
        if s.iter().all(|x| x[i]) {
            return i;
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let (r1, r2) = line.split_at(line.len() / 2);
        debug_assert_eq!(r1.len(), r2.len());
        let s1 = set_of(r1);
        let s2 = set_of(r2);
        sum += set_intersects([&s1, &s2]);
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for [g1, g2, g3] in input.lines().array_chunks::<3>() {
        let s1 = set_of(g1);
        let s2 = set_of(g2);
        let s3 = set_of(g3);
        sum += set_intersects([&s1, &s2, &s3]);
    }

    Some(sum as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
