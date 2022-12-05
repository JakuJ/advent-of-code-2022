#![feature(binary_heap_into_iter_sorted)]

pub fn part_one(input: &str) -> Option<u32> {
    let mut max: u32 = 0;
    let mut current: u32 = 0;

    for line in input.lines() {
        if let Ok(ration) = line.parse::<u32>() {
            current += ration;
        } else {
            if current > max {
                max = current;
            }
            current = 0;
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap = std::collections::BinaryHeap::new();
    let mut current: u32 = 0;

    for line in input.lines() {
        if let Ok(ration) = line.parse::<u32>() {
            current += ration;
        } else {
            heap.push(current);
            current = 0;
        }
    }

    heap.push(current);

    Some(heap.into_iter_sorted().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
