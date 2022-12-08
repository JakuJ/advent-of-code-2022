#![feature(test)]

pub fn part_one(input: &str) -> Option<u32> {
    let side = input.lines().next().unwrap().len();
    let mut bitmap = vec![0; side * side];

    for (y, line) in input.lines().enumerate() {
        // left to right
        let mut current: i32 = -1;
        for (x, c) in line.bytes().enumerate() {
            let h = (c - 48) as i32;
            if h > current {
                bitmap[x + y * side] = 1;
                current = h;
                if h == 9 {
                    break;
                }
            }
        }

        // right to left
        let mut current: i32 = -1;
        for (x, c) in line.bytes().rev().enumerate() {
            let h = (c - 48) as i32;
            if h > current {
                bitmap[(side - x - 1) + y * side] = 1;
                current = h;
                if h == 9 {
                    break;
                }
            }
        }
    }

    // top to bottom
    let mut current: Vec<i32> = vec![-1; side];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            let h = (c - 48) as i32;
            if h > current[x] {
                bitmap[x + y * side] = 1;
                current[x] = h;
            }
        }
    }

    // bottom to top
    let mut current: Vec<i32> = vec![-1; side];
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            let h = (c - 48) as i32;
            if h > current[x] {
                bitmap[x + (side - y - 1) * side] = 1;
                current[x] = h;
            }
        }
    }

    Some(bitmap.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let side = input.lines().next().unwrap().len();
    let mut scores = vec![(0, 0, 0, 0); side * side];

    let empty_seen = [0_usize; 10];

    // top to bottom
    let mut top_to_bottom = vec![empty_seen; side];
    for (y, line) in input.lines().enumerate() {
        // left to right
        let mut seen = empty_seen;
        for (x, c) in line.bytes().enumerate() {
            let h = (c - 48) as usize;
            // left to right
            scores[x + y * side].0 = (x - seen[h]) as u32;
            for s in seen.iter_mut().take(h + 1) {
                *s = x;
            }
            // top to bottom
            scores[x + y * side].1 = (y - top_to_bottom[x][h]) as u32;
            for s in top_to_bottom[x].iter_mut().take(h + 1) {
                *s = y;
            }
        }
    }

    // bottom to top
    let mut bottom_to_top = vec![empty_seen; side];
    for (ry, line) in input.lines().rev().enumerate() {
        // right to left
        let mut seen = empty_seen;
        for (rx, c) in line.bytes().rev().enumerate() {
            let h = (c - 48) as usize;
            // right to left
            scores[(side - rx - 1) + (side - ry - 1) * side].2 = (rx - seen[h]) as u32;
            for s in seen.iter_mut().take(h + 1) {
                *s = rx;
            }
            // bottom to top
            scores[(side - rx - 1) + (side - ry - 1) * side].3 = (ry - bottom_to_top[rx][h]) as u32;
            for s in bottom_to_top[rx].iter_mut().take(h + 1) {
                *s = ry;
            }
        }
    }

    scores.iter().map(|(a, b, c, d)| a * b * c * d).max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 8);
        assert_eq!(part_one(&input), Some(1843));
        assert_eq!(part_two(&input), Some(180000));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 8);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 8);
        b.iter(|| part_two(input));
    }
}
