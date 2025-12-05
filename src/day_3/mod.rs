fn get_joltage(line: &str) -> usize {
    let mut primero: Option<char> = None;
    let mut segundo: Option<char> = None;
    let line_iter = line.chars();
    let line_l = line.len();
    for (i, c) in line_iter.enumerate() {
        match (primero, segundo) {
            (None, None) => {
                primero = Some(c);
            }
            (Some(x), None) => {
                if c > x && i < (line_l - 1) {
                    primero = Some(c);
                } else {
                    segundo = Some(c);
                }
            }
            (Some(x), Some(y)) => {
                if c > x && i < (line_l - 1) {
                    primero = Some(c);
                    segundo = None;
                } else if c > x && i == (line_l - 1) || c > y {
                    segundo = Some(c);
                }
            }
            _ => {
                unreachable!("I've made a grave mistake");
            }
        }
    }

    let first = primero.unwrap().to_digit(10).unwrap() as usize;
    let second = segundo.unwrap().to_digit(10).unwrap() as usize;
    first * 10 + second
}

fn scan_for_next(line: &str, pos: usize, offset: usize) -> (usize, char) {
    let take = match (line.len() - offset) - pos > 0 {
        false => 1,
        true => (line.len() - offset) - pos,
    };
    let line_iter: String = line.chars().skip(offset).take(take).collect();
    let mut candidate: Option<(usize, char)> = None;
    for (i, c) in line_iter.chars().enumerate() {
        let real_i = i + offset;
        match candidate {
            None => {
                candidate = Some((real_i, c));
            }
            Some((_, x)) => {
                if c > x {
                    candidate = Some((real_i, c));
                }
            }
        }
    }
    candidate.unwrap()
}

fn get_joltage_2(line: &str) -> usize {
    let mut offset = 0;
    let mut res = String::new();
    for i in (0..12).rev() {
        let (n_offset, c) = scan_for_next(line, i, offset);
        offset = n_offset + 1;
        res.push(c);
    }
    res.parse::<usize>().ok().unwrap()
}

pub fn solve_pt1(input: &str) -> usize {
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(get_joltage)
        .sum()
}

pub fn solve_pt2(input: &str) -> usize {
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(get_joltage_2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_get_joltage() {
        let line = "4854535654444524444436464544444574714545533454364453534353525424355735354574314345565554335535555253";
        assert_eq!(get_joltage(line), 87);
    }

    #[test]
    fn test_get_joltage_2() {
        let line = "818181911112111";
        assert_eq!(get_joltage_2(line), 888911112111);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 357)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 3121910778619)
    }
}
