use std::{collections::HashSet, str::FromStr};

#[inline]
fn get_min_max(l: usize) -> usize {
    10_usize.pow((l - 1) as u32)
}

fn parse_line(line: &str) -> Vec<String> {
    line.split("-")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

#[inline]
fn format_n_seq(seq: String, n: usize) -> usize {
    seq.repeat(n).parse::<usize>().ok().unwrap()
}

#[inline]
fn get_seq_slice(full: &str, slice: usize) -> String {
    String::from_str(&full[..slice]).ok().unwrap()
}

fn add_dups(
    start: &str,
    end: &str,
    total: &mut usize,
    slice: usize,
    size: usize,
    seen: &mut HashSet<usize>,
) {
    let first_seq = get_seq_slice(start, slice);
    let max = end.parse::<usize>().ok().unwrap();
    let mut dup = format_n_seq(first_seq, size);
    while dup <= max {
        if dup >= start.parse::<usize>().ok().unwrap() && !seen.contains(&dup) {
            *total += dup;
            seen.insert(dup);
        }
        let dup_s = dup.to_string();
        let x = get_seq_slice(&dup_s, slice);
        let next_seq = x.parse::<usize>().ok().unwrap() + 1;
        dup = format_n_seq(next_seq.to_string(), size);
    }
}

fn get_unchecked_id_range(mut p: Vec<String>) -> (String, String) {
    let end = p.pop().unwrap();
    let start = p.pop().unwrap();
    (start, end)
}

fn get_checked_id_range(mut p: Vec<String>) -> Option<(String, String)> {
    let end = p.pop().unwrap();
    let start = p.pop().unwrap();
    let start_l = start.len();
    let end_l = end.len();
    let start_is_even = start_l.is_multiple_of(2);
    let end_is_even = end_l.is_multiple_of(2);
    if !(start_is_even || end_is_even) {
        None
    } else {
        let start = if !start_is_even {
            get_min_max(end_l)
        } else {
            start.parse::<usize>().ok().unwrap()
        };

        let end = if !end_is_even {
            get_min_max(start_l + 1) - 1
        } else {
            end.parse::<usize>().ok().unwrap()
        };

        Some((start.to_string(), end.to_string()))
    }
}

fn get_factors(n: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();
    for i in 1..((n / 2) + 1) {
        if n.is_multiple_of(i) {
            factors.push(i);
        }
    }
    factors
}

fn do_part_2_math(start: &str, end: &str, total: &mut usize) {
    let start_l = start.len();
    let end_l = end.len();
    let start_fs = get_factors(start_l);
    let mut seen = HashSet::new();
    for n in start_fs {
        add_dups(start, end, total, n, start_l / n, &mut seen);
    }
    if (!start_l.is_multiple_of(2) || !end_l.is_multiple_of(2)) && start_l != end_l {
        let start_2 = get_min_max(end_l).to_string();
        let start_2_l = start_2.len();
        let start_2_fs = get_factors(start_2_l);
        for n in start_2_fs {
            add_dups(&start_2, end, total, n, start_2_l / n, &mut seen);
        }
    }
}

pub fn solve_pt1(input: &str) -> usize {
    let mut total = 0;
    input
        .split(',')
        .take_while(|x| !x.is_empty())
        .for_each(|x| {
            let line = parse_line(x.trim());
            if let Some((start, end)) = get_checked_id_range(line) {
                let slice = start.len() / 2;
                let mut seen = HashSet::new();
                add_dups(
                    start.as_str(),
                    end.as_str(),
                    &mut total,
                    slice,
                    2,
                    &mut seen,
                );
            }
        });

    total
}

pub fn solve_pt2(input: &str) -> usize {
    let mut total = 0;
    input
        .split(',')
        .take_while(|x| !x.is_empty())
        .for_each(|x| {
            let line = parse_line(x.trim());
            let (start, end) = get_unchecked_id_range(line);
            do_part_2_math(start.as_str(), end.as_str(), &mut total);
        });

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str ="11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_get_check_id_range() {
        let start = "998".to_string();
        let end = "1010".to_string();
        let expected_start = "1000".to_string();
        let expected_end = "1010".to_string();
        assert_eq!(
            get_checked_id_range(vec![start, end]),
            Some((expected_start, expected_end))
        );
    }

    #[test]
    fn test_get_checked_id_range_2() {
        let start = "95".to_string();
        let end = "115".to_string();
        let expected_start = "95".to_string();
        let expected_end = "99".to_string();
        assert_eq!(
            get_checked_id_range(vec![start, end]),
            Some((expected_start, expected_end))
        );
    }

    #[test]
    fn test_add_dups() {
        let mut total = 0;
        let mut seen = HashSet::new();
        add_dups("38593856", "38593862", &mut total, 4, 2, &mut seen);
        assert_eq!(total, 38593859);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 1227775554)
    }

    #[test]
    fn test_get_factors() {
        assert_eq!(get_factors(9), vec![1, 3]);
    }

    #[test]
    fn test_do_part_2_math() {
        let mut total = 0;
        let start = "998";
        let end = "1012";
        do_part_2_math(start, end, &mut total);
        assert_eq!(total, 2009);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 4174379265);
    }
}
