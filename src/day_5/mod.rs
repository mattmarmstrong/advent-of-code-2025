use std::cmp::max;

const SPLIT: &str = "\n\n";

fn parse_and_solve(input: &str) -> usize {
    let mut iter = input.split(SPLIT);
    let fresh_i = iter.next().unwrap();
    let ids_i = iter.next().unwrap();
    let fresh: Vec<(usize, usize)> = fresh_i
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let mut line_i = line.split("-");
            let start = line_i.next().unwrap().parse::<usize>().ok().unwrap();
            let end = line_i.next().unwrap().parse::<usize>().ok().unwrap();
            (start, end)
        })
        .collect();
    ids_i
        .split("\n")
        .take_while(|x| !x.is_empty())
        .filter(|n| {
            let n = n.parse::<usize>().ok().unwrap();
            fresh.iter().any(|(x, y)| *x <= n && n <= *y)
        })
        .count()
}

fn reduce(mut v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut f = vec![v[0]];
    for (a, b) in v.iter() {
        let f_l = f.len();
        let mut l = f[f_l - 1];
        if *a <= l.1 + 1 {
            l.1 = max(l.1, *b);
            f[f_l - 1] = l;
        } else {
            f.push((*a, *b));
        }
    }
    f
}

fn parse_and_solve_2(input: &str) -> usize {
    let mut iter = input.split(SPLIT);
    let fresh_i = iter.next().unwrap();
    let fresh: Vec<(usize, usize)> = fresh_i
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let mut line_i = line.split("-");
            let start = line_i.next().unwrap().parse::<usize>().ok().unwrap();
            let end = line_i.next().unwrap().parse::<usize>().ok().unwrap();
            (start, end)
        })
        .collect();
    let f = reduce(fresh);
    let mut sum = 0;
    for (start, end) in f {
        sum += (end - start) + 1;
    }
    sum
}

pub fn solve_pt1(input: &str) -> usize {
    parse_and_solve(input)
}

pub fn solve_pt2(input: &str) -> usize {
    parse_and_solve_2(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 3)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 14)
    }
}
