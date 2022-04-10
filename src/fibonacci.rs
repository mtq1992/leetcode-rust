pub struct Solution;

impl Solution {
    pub fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => Solution::fibonacci(n-1) + Solution::fibonacci(n-2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_0() {
        assert_eq!(Solution::fibonacci(0), 1);
    }
    #[test]
    fn test_1() {
        assert_eq!(Solution::fibonacci(1), 1);
    }

    #[test]
    fn test_20() {
        assert_eq!(Solution::fibonacci(20), 10946);
    }
}
