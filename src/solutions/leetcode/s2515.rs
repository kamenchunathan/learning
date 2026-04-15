// 2515. Shortest Distance to Target String in a Circular Array
// You are given a 0-indexed circular string array words and a string target. A circular array means that the array's end connects to the array's beginning.
//
// Formally, the next element of words[i] is words[(i + 1) % n] and the previous element of words[i] is words[(i - 1 + n) % n], where n is the length of words.
// Starting from startIndex, you can move to either the next word or the previous word with 1 step at a time.
//
// Return the shortest distance needed to reach the string target. If the string target does not exist in words, return -1.
//
//
//
// Example 1:
//
// Input: words = ["hello","i","am","leetcode","hello"], target = "hello", startIndex = 1
// Output: 1
// Explanation: We start from index 1 and can reach "hello" by
// - moving 3 units to the right to reach index 4.
// - moving 2 units to the left to reach index 4.
// - moving 4 units to the right to reach index 0.
// - moving 1 unit to the left to reach index 0.
// The shortest distance to reach "hello" is 1.
// Example 2:
//
// Input: words = ["a","b","leetcode"], target = "leetcode", startIndex = 0
// Output: 1
// Explanation: We start from index 0 and can reach "leetcode" by
// - moving 2 units to the right to reach index 2.
// - moving 1 unit to the left to reach index 2.
// The shortest distance to reach "leetcode" is 1.
// Example 3:
//
// Input: words = ["i","eat","leetcode"], target = "ate", startIndex = 0
// Output: -1
// Explanation: Since "ate" does not exist in words, we return -1.
//
//
// Constraints:
//
// 1 <= words.length <= 100
// 1 <= words[i].length <= 100
// words[i] and target consist of only lowercase English letters.
// 0 <= startIndex < words.length
struct Solution;

impl Solution {
   pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len();
        words.into_iter().enumerate().filter(|(i, word)| { *word == target }).map(
            |(i, _)| {
                let dist = i32::abs((i as i32) - start_index) as usize;
                    (if dist <= n / 2 {
                        dist
                    }  else {
                        n - dist
                    }) as i32
            }
        ).min().unwrap_or(-1)
    } 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_1() {
        let q = vec!["hello", "i", "am", "leetcode", "hello"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("hello"), 1), 1);
    }

    #[test]
    pub fn example_2() {
        let q = vec!["a", "b", "leetcode"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("leetcode"), 0), 1);
    }

    #[test]
    pub fn example_3() {
        let q = vec!["i", "eat", "leetcode"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("ate"), 0), -1);
    }

    #[test]
    pub fn at_start_index() {
        let q = vec!["hello", "i", "am", "leetcode", "hello"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("i"), 1), 0);
    }

    #[test]
    pub fn single_word_match() {
        let q = vec!["hello"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("hello"), 0), 0);
    }

    #[test]
    pub fn single_word_no_match() {
        let q = vec!["hello"]
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();

        assert_eq!(Solution::closest_target(q, String::from("world"), 0), -1);
    }
}
