// 3300. Minimum Element After Replacement With Digit Sum
// You are given an integer array nums.
//
// You replace each element in nums with the sum of its digits.
//
// Return the minimum element in nums after all replacements.
//
//
//
// Example 1:
//
// Input: nums = [10,12,13,14]
//
// Output: 1
//
// Explanation:
//
// nums becomes [1, 3, 4, 5] after all replacements, with minimum element 1.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
//
// Output: 1
//
// Explanation:
//
// nums becomes [1, 2, 3, 4] after all replacements, with minimum element 1.
//
// Example 3:
//
// Input: nums = [999,19,199]
//
// Output: 10
//
// Explanation:
//
// nums becomes [27, 10, 19] after all replacements, with minimum element 10.
//
//
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 104

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|num| {
                let mut n = num;
                let mut acc = 0;

                while n / 10 > 0 {
                    acc += n % 10;
                    n = n / 10;
                }

                acc + n
            })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_1() {
        assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
    }

    #[test]
    pub fn example_2() {
        assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    pub fn example_3() {
        assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
    }

    #[test]
    pub fn single_element() {
        assert_eq!(Solution::min_element(vec![5]), 5);
    }

    #[test]
    pub fn single_digit_numbers() {
        assert_eq!(Solution::min_element(vec![9, 8, 7, 6]), 6);
    }

    #[test]
    pub fn large_numbers() {
        assert_eq!(Solution::min_element(vec![9999, 8888, 7777]), 28);
    }

    #[test]
    pub fn all_same_digit_sum() {
        assert_eq!(Solution::min_element(vec![19, 28, 37, 46]), 10);
    }
}
