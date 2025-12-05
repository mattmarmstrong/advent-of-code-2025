fn parse_line_pt1(line: &str) -> (char, u64) {
    let mut line_chars = line.chars().collect::<Vec<char>>().into_iter();
    let dir = line_chars.next().unwrap();
    let count: u64 = line_chars
        .rev()
        .take(2)
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (dir, count)
}

fn parse_line_pt2(line: &str) -> (char, u64) {
    let mut line_chars = line.chars().collect::<Vec<char>>().into_iter();
    let dir = line_chars.next().unwrap();
    let count: u64 = line_chars
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (dir, count)
}

fn turn_left(count: u64, curr: &mut i64) -> bool {
    let curr_v = *curr;
    let count = count as i64;
    if (curr_v - count) > 0 {
        *curr -= count;
        false
    } else {
        let z = curr_v == 0;
        let wrap = curr_v.abs_diff(count) as i64;
        if wrap == 0 {
            *curr = 0;
            return true;
        } else {
            *curr = 100 - wrap
        }
        !z
    }
}

fn turn_right(count: u64, curr: &mut i64) -> bool {
    let curr_v = *curr;
    let count = count as i64;
    if (curr_v + count) >= 100 {
        *curr = (curr_v + count) - 100;
        true
    } else {
        *curr += count;
        false
    }
}

fn do_math(dir: char, count: u64, curr: &mut i64) -> bool {
    match dir {
        'L' => turn_left(count, curr),
        'R' => turn_right(count, curr),
        _ => {
            panic!("Bad Input!")
        }
    }
}

fn do_math_2(dir: char, count: u64, curr: &mut i64) -> u64 {
    let mut total = count / 100;
    let remainder = count % 100;
    if do_math(dir, remainder, curr) {
        total += 1;
    }
    total
}

pub fn solve_pt1(input: &str) -> u64 {
    let mut curr: i64 = 50;
    let mut total: u64 = 0;
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .for_each(|line| {
            let (dir, count) = parse_line_pt1(line);
            do_math(dir, count, &mut curr);
            if curr == 0 {
                total += 1;
            }
        });
    total
}

pub fn solve_pt2(input: &str) -> u64 {
    let mut curr: i64 = 50;
    let mut total: u64 = 0;
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .for_each(|line| {
            let (dir, count) = parse_line_pt2(line);
            let x = do_math_2(dir, count, &mut curr);
            total += x;
        });
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 3)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 6)
    }

    #[test]
    fn test_pt2_turn_left() {
        let expected = 6;
        let mut curr = 37;
        let line = "L537";
        let (dir, count) = parse_line_pt2(line);
        assert_eq!(dir, 'L');
        assert_eq!(count, 537);

        let actual = do_math_2(dir, count, &mut curr);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_pt2_turn_right() {
        let expected = 11;
        let mut curr = 37;
        let line = "R1063";
        let (dir, count) = parse_line_pt2(line);
        assert_eq!(dir, 'R');
        assert_eq!(count, 1063);

        let actual = do_math_2(dir, count, &mut curr);
        assert_eq!(actual, expected)
    }
}
