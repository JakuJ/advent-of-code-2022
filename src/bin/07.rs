#![feature(test)]

fn process<F>(input: &str, init: u32, update: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    let mut stack: Vec<u32> = Vec::new();
    let mut acc = init;

    for line in input.lines() {
        if line.starts_with("$") {
            // parse command
            let (_, line) = line.split_at(2);
            if line.len() > 2 {
                if line == "cd .." {
                    // add to current directory
                    let size = stack.pop().unwrap();
                    // update the accumulator
                    acc = update(acc, size);
                    // add to the outer directory
                    *stack.last_mut().unwrap() += size;
                } else {
                    // start counting in a new directory
                    stack.push(0);
                }
            }
        } else if !line.starts_with("d") {
            // count file size towards current directory
            let (digits, _) = line.split_once(" ").unwrap();
            *stack.last_mut().unwrap() += digits.parse::<u32>().unwrap();
        }
    }

    // remember to process the root directory
    while let Some(size) = stack.pop() {
        acc = update(acc, size);

        if let Some(last) = stack.last_mut() {
            *last += size;
        }
    }

    acc
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process(input, 0, |acc, size| {
        if size <= 100000 {
            acc + size
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let root_size = process(input, 0, |_, x| x);

    let space_left = 70000000 - root_size;

    Some(process(input, root_size, |acc, size| {
        if space_left + size >= 30000000 && size < acc {
            size
        } else {
            acc
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 7);
        assert_eq!(part_one(&input), Some(1391690));
        assert_eq!(part_two(&input), Some(5469168));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 7);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 7);
        b.iter(|| part_two(input));
    }
}
