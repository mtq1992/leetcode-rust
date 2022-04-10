

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i]+nums[j] == target {
                    return vec![i as i32,j as i32]
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(j) = map.get(&complement) {
                return vec![*j, i as i32];
            }
            map.insert(nums[i], i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
    
    #[test]
    fn test_hash() {
        assert_eq!(Solution::two_sum_hash(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}


#[cfg(test)]
mod benchs {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn bench_two_sum(b: &mut Bencher) {
        b.iter(|| Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[bench]
    fn bench_two_sum_hash(b: &mut Bencher) {
        b.iter(|| Solution::two_sum_hash(vec![2, 7, 11, 15], 9));
    }
}


