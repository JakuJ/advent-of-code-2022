fn play(p1: char, p2: char) -> u32 {
    // shape score
    let score = match p2 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    // match outcome
    score
        + match (p1, p2) {
            // draw
            ('A', 'X') => 3,
            ('B', 'Y') => 3,
            ('C', 'Z') => 3,
            // win
            ('A', 'Y') => 6,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            // lose
            _ => 0,
        }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| play(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
            .sum(),
    )
}

fn strategy(p1: char, outcome: char) -> u32 {
    let my_move = match (p1, outcome) {
        // lose
        ('A', 'X') => 'Z',
        ('B', 'X') => 'X',
        ('C', 'X') => 'Y',
        // draw
        ('A', 'Y') => 'X',
        ('B', 'Y') => 'Y',
        ('C', 'Y') => 'Z',
        // win
        ('A', 'Z') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'Z') => 'X',
        _ => unreachable!(),
    };

    play(p1, my_move)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| strategy(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
