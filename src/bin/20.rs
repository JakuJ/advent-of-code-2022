#![feature(test)]

pub fn part_one(input: &str) -> Option<i32> {
    let nums = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let len = nums.len() as i32;

    let mut mixed2orig = Vec::from_iter(0..len);
    let mut orig2mixed = Vec::from_iter(0..len);

    for (orig_ix, val) in nums.iter().enumerate() {
        let mixed_ix = orig2mixed[orig_ix];
        let mut new_mixed = (mixed_ix + *val).rem_euclid(len - 1);

        if new_mixed == 0 {
            new_mixed = len - 1;
        }

        for (o, m) in orig2mixed.iter_mut().enumerate() {
            if o != orig_ix {
                if *m > mixed_ix {
                    *m -= 1;
                }
                if *m >= new_mixed {
                    *m += 1;
                }
                mixed2orig[*m as usize] = o as i32;
            }
        }

        orig2mixed[orig_ix] = new_mixed;
        mixed2orig[new_mixed as usize] = orig_ix as i32;
    }

    let len = len as usize;
    let zero_ix = orig2mixed[nums.iter().enumerate().find(|(_, x)| **x == 0).unwrap().0] as usize;
    let ix_fst = (zero_ix + 1000) % len;
    let ix_snd = (zero_ix + 2000) % len;
    let ix_trd = (zero_ix + 3000) % len;
    Some(
        nums[mixed2orig[ix_fst] as usize]
            + nums[mixed2orig[ix_snd] as usize]
            + nums[mixed2orig[ix_trd] as usize],
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let nums = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * 811589153)
        .collect::<Vec<_>>();

    let len = nums.len() as i64;

    let mut mixed2orig = Vec::from_iter(0..len);
    let mut orig2mixed = Vec::from_iter(0..len);

    for _ in 0..10 {
        for (orig_ix, val) in nums.iter().enumerate() {
            let mixed_ix = orig2mixed[orig_ix];
            let mut new_mixed = (mixed_ix + *val).rem_euclid(len - 1);

            if new_mixed == 0 {
                new_mixed = len - 1;
            }

            for (o, m) in orig2mixed.iter_mut().enumerate() {
                if o != orig_ix {
                    if *m > mixed_ix {
                        *m -= 1;
                    }
                    if *m >= new_mixed {
                        *m += 1;
                    }
                    mixed2orig[*m as usize] = o as i64;
                }
            }

            orig2mixed[orig_ix] = new_mixed;
            mixed2orig[new_mixed as usize] = orig_ix as i64;
        }
    }

    let len = len as usize;
    let zero_ix = orig2mixed[nums.iter().enumerate().find(|(_, x)| **x == 0).unwrap().0] as usize;
    let ix_fst = (zero_ix + 1000) % len;
    let ix_snd = (zero_ix + 2000) % len;
    let ix_trd = (zero_ix + 3000) % len;
    Some(
        nums[mixed2orig[ix_fst] as usize]
            + nums[mixed2orig[ix_snd] as usize]
            + nums[mixed2orig[ix_trd] as usize],
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 20);
        assert_eq!(part_one(&input), Some(2203));
        assert_eq!(part_two(&input), Some(6641234038999));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 20);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 20);
        b.iter(|| part_two(input));
    }
}
