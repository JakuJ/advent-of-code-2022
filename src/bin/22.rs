#![feature(test)]

#[derive(Debug)]
enum Command {
    Forward(u32),
    Turn(char),
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let lines = input.lines().collect::<Vec<_>>();

    let (lines, path) = lines.split_at(lines.len() - 2);

    let width = lines.iter().map(|x| x.len()).max().unwrap();
    let height = lines.len();

    let mut map = vec![vec![' '; width + 2]; height + 2];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y + 1][x + 1] = c;
        }
    }

    let path = path[1];

    let mut commands = Vec::with_capacity(path.len() / 2);
    let mut num = String::with_capacity(8);
    for c in path.chars() {
        match c {
            '0'..='9' => {
                num.push(c);
            }
            'L' | 'R' => {
                commands.push(Command::Forward(num.parse().unwrap()));
                num.clear();
                commands.push(Command::Turn(c))
            }
            _ => unreachable!(),
        }
    }

    if !num.is_empty() {
        commands.push(Command::Forward(num.parse().unwrap()));
    }

    (map, commands)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (map, commands) = parse(input);

    // find starting point
    let mut r = 1;
    let mut c = 1;

    'search: for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == '.' {
                (r, c) = (y as i32, x as i32);
                break 'search;
            }
        }
    }

    let invalid = |y: i32, x: i32| map[y as usize][x as usize] == ' ';

    // traverse the map
    let mut direction = (1, 0);

    for command in commands.iter() {
        match command {
            Command::Turn('R') => {
                direction = (-direction.1, direction.0);
            }
            Command::Turn('L') => {
                direction = (direction.1, -direction.0);
            }
            &Command::Forward(steps) => {
                for _ in 0..steps {
                    let (old_r, old_c) = (r, c);

                    // make one step in direction
                    (r, c) = (r + direction.1, c + direction.0);

                    // check if we are out of bounds
                    if invalid(r, c) {
                        // go as far as possible in the oposite direction until ' ' encountered
                        while !invalid(r - direction.1, c - direction.0) {
                            c -= direction.0;
                            r -= direction.1;
                        }
                    }
                    // check if we are standing on top of an obstacle ('#')
                    if map[r as usize][c as usize] == '#' {
                        (r, c) = (old_r, old_c);
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let facing = match direction {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => unreachable!(),
    };

    Some(r * 1000 + c * 4 + facing)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 22);
        assert_eq!(part_one(&input), Some(165094));
        assert_eq!(part_two(&input), None);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 22);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 22);
        b.iter(|| part_two(input));
    }
}
