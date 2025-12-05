pub fn solve_pt1(input: &str) -> u64 {}

pub fn solve_pt2(input: &str) -> u64 {}

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
}
