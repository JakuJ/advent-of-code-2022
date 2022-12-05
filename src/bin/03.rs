#![feature(iter_array_chunks)]

fn priority(x: u8) -> u8 {
    if x <= 90 {
        x - 38
    } else {
        x - 96
    }
}

type Set = u64;

fn set_of(rucksack: &str) -> Set {
    rucksack.bytes().fold(0, |acc, e| acc | (1 << priority(e)))
}

fn set_intersects(s: &[Set]) -> u32 {
    s.iter().fold(Set::MAX, |acc, &e| acc & e).trailing_zeros()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|g| set_intersects(&[set_of(g.0), set_of(g.1)]))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .array_chunks::<3>()
            .map(|g| set_intersects(&g.map(set_of)))
            .sum(),
    )
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
        assert_eq!(priority(b'a'), 1);
        assert_eq!(priority(b'z'), 26);
        assert_eq!(priority(b'A'), 27);
        assert_eq!(priority(b'Z'), 52);
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

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 3);
        assert_eq!(part_one(&input), Some(7908));
        assert_eq!(part_two(&input), Some(2838));
    }
}
