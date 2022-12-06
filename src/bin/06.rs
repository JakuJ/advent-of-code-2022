#![feature(test)]

fn encode(x: u8) -> u64 {
    1 << (x - 97)
}

fn solve_n<const N: usize>(input: &str) -> Option<usize> {
    let mut buf: [u64; N] = [0; N];

    let (left, right) = input.split_at(N - 1);

    for (i, c) in left.bytes().enumerate() {
        buf[i] = encode(c);
    }

    let mut ptr = N - 1;

    for (ix, c) in right.bytes().enumerate() {
        buf[ptr] = encode(c);

        if buf.iter().fold(0, |acc, &x| acc | x).count_ones() == N as u32 {
            return Some(ix + N);
        }

        ptr = (ptr + 1) % N;
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_n::<4>(input)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_n::<14>(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 6);
        assert_eq!(part_one(&input), Some(1804));
        assert_eq!(part_two(&input), Some(2508));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 6);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 6);
        b.iter(|| part_two(input));
    }
}
