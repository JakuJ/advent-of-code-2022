#![feature(test)]
#![feature(iter_intersperse)]

const WIDTH: usize = 40;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

const LIT: char = '#';
const DARK: char = '.';

pub fn part_one(input: &str) -> Option<i32> {
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut signal: i32 = 0;

    for line in input.lines() {
        let (_, value) = line.split_at(4);

        if (cycle - 20) % 40 == 0 {
            signal += cycle * register;
        }
        cycle += 1;

        if !value.is_empty() {
            if (cycle - 20) % 40 == 0 {
                signal += cycle * register;
            }
            cycle += 1;
            register += value[1..].parse::<i32>().unwrap();
        }
    }

    Some(signal)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut display = [DARK; SIZE];
    let mut renderer: usize = 0;

    let mut render = |reg| {
        display[renderer] = if (renderer % WIDTH).abs_diff(reg as usize) <= 1 {
            LIT
        } else {
            DARK
        };

        renderer = (renderer + 1) % SIZE;

        cycle += 1;
    };

    for line in input.lines() {
        let (_, value) = line.split_at(4);

        render(register);

        if !value.is_empty() {
            render(register);
            register += value[1..].parse::<i32>().unwrap();
        }
    }

    Some(
        display
            .chunks(WIDTH)
            .intersperse(&['\n'])
            .flatten()
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                    .to_string()
            )
        );
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 10);
        assert_eq!(part_one(&input), Some(14240));
        assert_eq!(
            part_two(&input),
            Some(
                "###..#....#..#.#....#..#.###..####.#..#.
...#.#....#..#.#....#.#..#..#....#.#..#.
#..#.#....#..#.#....##...###....#..####.
###..#....#..#.#....#.#..#..#..#...#..#.
.....#....#..#.#....#.#..#..#.#....#..#.
.....####..##..####.#..#.###..####.#..#."
                    .to_string()
            )
        );
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 10);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 10);
        b.iter(|| part_two(input));
    }
}
