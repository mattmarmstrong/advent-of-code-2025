fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect()
}

fn parse_2(input: &str) -> Vec<String> {
    input
        .split("\n")
        .take_while(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn do_work(p_input: Vec<Vec<&str>>) -> usize {
    let y_max = p_input.len() - 1;
    let x_max = p_input[0].len();
    let mut total = 0;
    for i in 0..x_max {
        let mut s_total = 1;
        let operator = p_input[y_max][i];
        for j in 0..(y_max) {
            let v = p_input[j][i].parse::<usize>().ok().unwrap();
            match operator {
                "+" => total += v,
                "*" => s_total *= v,
                _ => unreachable!("pure pandemonium"),
            }
        }
        if s_total > 1 {
            total += s_total;
        }
    }
    total
}

#[allow(clippy::needless_range_loop)]
fn transform(mut p_input: Vec<String>) -> Vec<Vec<String>> {
    let operators_str = p_input.pop().unwrap();
    let operators: Vec<&str> = operators_str
        .split_whitespace()
        .take_while(|x| !x.is_empty())
        .collect();

    let x_max = p_input[0].len();
    let y_max = p_input.len();
    let mut res: Vec<Vec<String>> = Vec::with_capacity(y_max + 1);
    let mut x = Vec::new();
    for i in 0..x_max {
        let mut s = String::new();
        for j in 0..y_max {
            let char = p_input[j].chars().nth(i).unwrap();
            match char.is_whitespace() {
                true => continue,
                false => s.push(char),
            }
        }
        if !s.is_empty() {
            x.push(s);
        } else {
            res.push(x);
            x = Vec::new();
        }
    }
    res.push(x);
    for i in 0..operators.len() {
        res[i].push(operators[i].to_string());
    }
    res
}

fn do_work_2(mut x: Vec<Vec<String>>) -> usize {
    let mut total = 0;
    for line in x.iter_mut() {
        let mut s_total = 1;
        let operator = line.last().unwrap().as_str();
        for item in line.iter().rev().skip(1).rev() {
            let v = item.parse::<usize>().ok().unwrap();
            match operator {
                "+" => total += v,
                "*" => s_total *= v,
                _ => unreachable!("pure pandemonium 2"),
            }
        }
        if s_total > 1 {
            total += s_total;
        }
    }
    total
}

pub fn solve_pt1(input: &str) -> usize {
    let v = parse(input);
    do_work(v)
}

pub fn solve_pt2(input: &str) -> usize {
    let v = parse_2(input);
    let p_v = transform(v);
    do_work_2(p_v)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_pt1() {
        assert_eq!(solve_pt1(EXAMPLE), 4277556)
    }

    #[test]
    fn test_pt2() {
        assert_eq!(solve_pt2(EXAMPLE), 3263827)
    }
}
