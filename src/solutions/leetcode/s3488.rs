// 3488. Closest Equal Element Queries
//
// You are given a circular array nums and an array queries.
//
// For each query i, you have to find the following:
//
// The minimum distance between the element at index queries[i] and any other index j in the circular array, where nums[j] == nums[queries[i]]. If no such index exists, the answer for that query should be -1.
// Return an array answer of the same size as queries, where answer[i] represents the result for query i.
//
//
//
// Example 1:
//
// Input: nums = [1,3,1,4,1,3,2], queries = [0,3,5]
//
// Output: [2,-1,3]
//
//[1,3,1,4,1,3,2]
//[0,1,2,3,4,5,6]
//[0,2,4,6,1,5,3]
//
// Explanation:
//
// Query 0: The element at queries[0] = 0 is nums[0] = 1. The nearest index with the same value is 2, and the distance between them is 2.
// Query 1: The element at queries[1] = 3 is nums[3] = 4. No other index contains 4, so the result is -1.
// Query 2: The element at queries[2] = 5 is nums[5] = 3. The nearest index with the same value is 1, and the distance between them is 3 (following the circular path: 5 -> 6 -> 0 -> 1).
// Example 2:
//
// Input: nums = [1,2,3,4], queries = [0,1,2,3]
//
// Output: [-1,-1,-1,-1]
//
// Explanation:
//
// Each value in nums is unique, so no index shares the same value as the queried element. This results in -1 for all queries.
//
//
//
// Constraints:
//
// 1 <= queries.length <= nums.length <= 105
// 1 <= nums[i] <= 106
// 0 <= queries[i] < nums.length
struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut indices = (0..).take(nums.len()).collect::<Vec<usize>>();
        indices.sort_by_key(|&i| nums[i]);
        println!("{indices:?}");

        queries
            .iter()
            .map(|&q| {
                indices
                    .binary_search_by_key(&nums[q as usize], |&i| nums[i])
                    .ok()
                    .map(|k| {
                        let a: usize;
                        let b: usize;

                        // Left
                        let mut c = 0;
                        while k >= c && nums[indices[k - c]] == nums[q as usize] {
                            c += 1;
                        }
                        a = k + 1 - c;

                        // Right
                        c = 1;
                        while c + k < indices.len() && nums[indices[k + c]] == nums[q as usize] {
                            c += 1;
                        }
                        b = k + c;

                        indices[a..b]
                            .iter()
                            .filter(|&&j| j as i32 != q)
                            .map(|&j| {
                                let dist = i32::abs(j as i32 - q);
                                if dist <= (nums.len() / 2) as i32 {
                                    dist
                                } else {
                                    nums.len() as i32 - dist
                                }
                            })
                            .min()
                            .clone()
                    })
                    .flatten()
                    .unwrap_or(-1)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_1() {
        let sol = Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]);
        assert_eq!(sol, vec![2, -1, 3]);
    }

    #[test]
    pub fn example_2() {
        let sol = Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]);
        assert_eq!(sol, vec![-1, -1, -1, -1]);
    }

    #[test]
    pub fn test_two_elements() {
        // Distance in circular array [1, 1] is always 1
        let sol = Solution::solve_queries(vec![1, 1], vec![0, 1]);
        assert_eq!(sol, vec![1, 1]);
    }

    #[test]
    pub fn test_all_same() {
        // [1, 1, 1, 1, 1] - n=5. Distances should be 1.
        let sol = Solution::solve_queries(vec![1, 1, 1, 1, 1], vec![0, 2, 4]);
        assert_eq!(sol, vec![1, 1, 1]);
    }

    #[test]
    pub fn test_circular_wrap() {
        // [1, 2, 2, 1], q=0 (nums[0]=1). Other 1 is at index 3.
        // Distance: abs(0-3) = 3. Circular: 4-3 = 1.
        let sol = Solution::solve_queries(vec![1, 2, 2, 1], vec![0, 3]);
        assert_eq!(sol, vec![1, 1]);
    }

    #[test]
    pub fn test_boundary_indices() {
        // Test when the target value is at the very beginning or end of the sorted indices array
        let sol = Solution::solve_queries(vec![1, 2, 3, 1], vec![0, 3]);
        assert_eq!(sol, vec![1, 1]);
    }
}
