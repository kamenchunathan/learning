// 3121. Count the Number of Special Characters II
// // You are given a string word. A letter c is called special if it appears both in lowercase and uppercase in word, and every lowercase occurrence of c appears before the first uppercase occurrence of c.
//
// Return the number of special letters in word.
//
//
//
// Example 1:
//
// Input: word = "aaAbcBC"
//
// Output: 3
//
// Explanation:
//
// The special characters are 'a', 'b', and 'c'.
//
// Example 2:
//
// Input: word = "abc"
//
// Output: 0
//
// Explanation:
//
// There are no special characters in word.
//
// Example 3:
//
// Input: word = "AbBCab"
//
// Output: 0
//
// Explanation:
//
// There are no special characters in word.
//
//
//
// Constraints:
//
// 1 <= word.length <= 2 * 105
// word consists of only lowercase and uppercase English letters.

pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        Self::number_of_special_chars_v3(word)
    }

    // Solution 1: Array-based approach
    pub fn number_of_special_chars_v1(word: String) -> i32 {
        let mut lower = [false; 26];
        let mut upper = [false; 26];

        for c in word.chars() {
            if c.is_ascii_lowercase() {
                let idx = c as usize - 97;
                if upper[idx] {
                    lower[idx] = false;
                } else {
                    lower[idx] = true;
                }
            } else if c.is_ascii_uppercase() {
                let idx = c as usize - 65;
                upper[idx] = true;
            }
        }

        let mut count = 0;
        for i in 0..26 {
            if lower[i] && upper[i] {
                count += 1;
            }
        }

        count
    }

    // Solution 2: Bitwise approach using match range
    pub fn number_of_special_chars_v2(word: String) -> i32 {
        let mut lower_bits: u32 = 0;
        let mut upper_bits: u32 = 0;

        for c in word.bytes() {
            match c {
                b'a'..=b'z' => {
                    let bit = 1 << (c - b'a');
                    if (upper_bits & bit) != 0 {
                        lower_bits &= !bit;
                    } else {
                        lower_bits |= bit;
                    }
                }
                b'A'..=b'Z' => {
                    upper_bits |= 1 << (c - b'A');
                }
                _ => {}
            }
        }

        (upper_bits & lower_bits).count_ones() as i32
    }

    // Solution 3: Optimized branchless bitwise approach using ASCII tricks
    pub fn number_of_special_chars_v3(word: String) -> i32 {
        let mut lower_bits: u32 = 0;
        let mut upper_bits: u32 = 0;

        for c in word.bytes() {
            let bit = 1 << ((c & 31) - 1);
            if (c & 32) == 0 {
                upper_bits |= bit;
            } else {
                lower_bits = (lower_bits | bit) & !(upper_bits & bit);
            }
        }

        (upper_bits & lower_bits).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(word: &str, expected: i32) {
        assert_eq!(
            Solution::number_of_special_chars_v1(word.to_string()),
            expected,
            "v1 failed for {}",
            word
        );
        assert_eq!(
            Solution::number_of_special_chars_v2(word.to_string()),
            expected,
            "v2 failed for {}",
            word
        );
        assert_eq!(
            Solution::number_of_special_chars_v3(word.to_string()),
            expected,
            "v3 failed for {}",
            word
        );
        assert_eq!(
            Solution::number_of_special_chars(word.to_string()),
            expected,
            "default failed for {}",
            word
        );
    }

    #[test]
    fn test_example_1() {
        check("aaAbcBC", 3);
    }

    #[test]
    fn test_example_2() {
        check("abc", 0);
    }

    #[test]
    fn test_example_3() {
        check("AbBCab", 0);
    }

    #[test]
    fn yikes() {
        check("bBb", 0);
    }

    #[test]
    fn test_all_lowercase() {
        check("abcdefghijklmnopqrstuvwxyz", 0);
    }

    #[test]
    fn test_all_uppercase() {
        check("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0);
    }

    #[test]
    fn test_empty() {
        check("", 0);
    }

    #[test]
    fn test_single_lowercase() {
        check("a", 0);
    }

    #[test]
    fn test_single_uppercase() {
        check("A", 0);
    }

    #[test]
    fn test_valid_pair() {
        check("aA", 1);
    }

    #[test]
    fn test_invalid_pair_order() {
        check("Aa", 0);
    }
}
