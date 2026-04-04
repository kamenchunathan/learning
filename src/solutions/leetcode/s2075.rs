// A string originalText is encoded using a slanted transposition cipher to a string encodedText with the help of a matrix having a fixed number of rows rows.
//
// originalText is placed first in a top-left to bottom-right manner.
//
// The blue cells are filled first, followed by the red cells, then the yellow cells, and so on, until we reach the end of originalText. The arrow indicates the order in which the cells are filled. All empty cells are filled with ' '. The number of columns is chosen such that the rightmost column will not be empty after filling in originalText.
//
// encodedText is then formed by appending all characters of the matrix in a row-wise fashion.
//
// The characters in the blue cells are appended first to encodedText, then the red cells, and so on, and finally the yellow cells. The arrow indicates the order in which the cells are accessed.
//
// For example, if originalText = "cipher" and rows = 3, then we encode it in the following manner:
//
// The blue arrows depict how originalText is placed in the matrix, and the red arrows denote the order in which encodedText is formed. In the above example, encodedText = "ch ie pr".
//
// Given the encoded string encodedText and number of rows rows, return the original string originalText.
//
// Note: originalText does not have any trailing spaces ' '. The test cases are generated such that there is only one possible originalText.
//
//
//
// Example 1:
//
// Input: encodedText = "ch   ie   pr", rows = 3
// Output: "cipher"
// Explanation: This is the same example described in the problem description.
//
// Example 2:
//
// Input: encodedText = "iveo    eed   l te   olc", rows = 4
// Output: "i love leetcode"
// Explanation: The figure above denotes the matrix that was used to encode originalText.
// The blue arrows show how we can find originalText from encodedText.
//
// Example 3:
//
// Input: encodedText = "coding", rows = 1
// Output: "coding"
// Explanation: Since there is only 1 row, both originalText and encodedText are the same.
//
//
//
// Constraints:
//
//     0 <= encodedText.length <= 106
//     encodedText consists of lowercase English letters and ' ' only.
//     encodedText is a valid encoding of some originalText that does not have trailing spaces.
//     1 <= rows <= 1000
//     The testcases are generated such that there is only one possible originalText.
//

use crate::dlog;

struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        if encoded_text.is_empty() {
            return String::new();
        }
        // len is okay because the test is guaratneed to be ASCII which should be a byte per char
        let cols = encoded_text.len() as u32 / rows as u32;
        let rows = rows as u32;
        let encoded_text = encoded_text.into_bytes();
        let orig_txt_bounding_len = dbg!(rows * (dbg!(cols) - dbg!(rows) + 2));
        let mut orig_txt = Vec::with_capacity(orig_txt_bounding_len as usize);

        for i in 0..(orig_txt_bounding_len) {
            let (x, y) = to_slanted_coords(i, rows);
            let encoded_idx = grid_coords_to_idx((y, x), cols) as usize;
            if encoded_idx >= encoded_text.len() {
                continue;
            }
            orig_txt.insert(i as usize, encoded_text[encoded_idx]);
        }
        String::from_utf8(orig_txt).unwrap().trim_end().to_string()
    }
}

#[inline]
fn to_slanted_coords(i: u32, rows: u32) -> (u32, u32) {
    let y = i % rows;
    let x = i / rows + y;

    (x, y)
}

#[inline]
fn grid_coords_to_idx((x, y): (u32, u32), rows: u32) -> u32 {
    x * rows + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_1() {
        let sol = Solution::decode_ciphertext("ch   ie   pr".to_string(), 3);
        assert_eq!(sol, "cipher");
    }

    #[test]
    pub fn test_2() {
        let sol = Solution::decode_ciphertext("jna   oyv   nbo   nr".to_string(), 4);
        assert_eq!(sol, "jonnnybravo");
    }
}
