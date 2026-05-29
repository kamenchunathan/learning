// 3093. Longest Common Suffix Queries
// You are given two arrays of strings wordsContainer and wordsQuery.
//
// For each wordsQuery[i], you need to find a string from wordsContainer that has the longest common suffix with wordsQuery[i]. If there are two or more strings in wordsContainer that share the longest common suffix, find the string that is the smallest in length. If there are two or more such strings that have the same smallest length, find the one that occurred earlier in wordsContainer.
//
// Return an array of integers ans, where ans[i] is the index of the string in wordsContainer that has the longest common suffix with wordsQuery[i].
//
//
//
// Example 1:
//
// Input: wordsContainer = ["abcd","bcd","xbcd"], wordsQuery = ["cd","bcd","xyz"]
//
// Output: [1,1,1]
//
// Explanation:
//
// Let's look at each wordsQuery[i] separately:
//
// For wordsQuery[0] = "cd", strings from wordsContainer that share the longest common suffix "cd" are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
// For wordsQuery[1] = "bcd", strings from wordsContainer that share the longest common suffix "bcd" are at indices 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
// For wordsQuery[2] = "xyz", there is no string from wordsContainer that shares a common suffix. Hence the longest common suffix is "", that is shared with strings at index 0, 1, and 2. Among these, the answer is the string at index 1 because it has the shortest length of 3.
// Example 2:
//
// Input: wordsContainer = ["abcdefgh","poiuygh","ghghgh"], wordsQuery = ["gh","acbfgh","acbfegh"]
//
// Output: [2,0,2]
//
// Explanation:
//
// Let's look at each wordsQuery[i] separately:
//
// For wordsQuery[0] = "gh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
// For wordsQuery[1] = "acbfgh", only the string at index 0 shares the longest common suffix "fgh". Hence it is the answer, even though the string at index 2 is shorter.
// For wordsQuery[2] = "acbfegh", strings from wordsContainer that share the longest common suffix "gh" are at indices 0, 1, and 2. Among these, the answer is the string at index 2 because it has the shortest length of 6.
//
//
// Constraints:
//
// 1 <= wordsContainer.length, wordsQuery.length <= 104
// 1 <= wordsContainer[i].length <= 5 * 103
// 1 <= wordsQuery[i].length <= 5 * 103
// wordsContainer[i] consists only of lowercase English letters.
// wordsQuery[i] consists only of lowercase English letters.
// Sum of wordsContainer[i].length is at most 5 * 105.
// Sum of wordsQuery[i].length is at most 5 * 105.

pub struct SuffixTrieNode {
    children: [Option<Box<SuffixTrieNode>>; 26],
    best_index: usize,
    best_len: usize,
}

pub struct SuffixTrie {
    root: SuffixTrieNode,
}

impl SuffixTrie {
    pub fn from_container(words: &[String]) -> Self {
        let mut trie = SuffixTrie {
            root: SuffixTrieNode {
                children: Default::default(),
                best_index: words.len(),
                best_len: usize::MAX,
            },
        };
        for (i, word) in words.iter().enumerate() {
            let mut node = &mut trie.root;
            if word.len() < node.best_len || (word.len() == node.best_len && i < node.best_index) {
                node.best_index = i;
                node.best_len = word.len();
            }
            for b in word.bytes().rev() {
                let idx = (b - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| {
                    Box::new(SuffixTrieNode {
                        children: Default::default(),
                        best_index: i,
                        best_len: word.len(),
                    })
                });
                if word.len() < node.best_len
                    || (word.len() == node.best_len && i < node.best_index)
                {
                    node.best_index = i;
                    node.best_len = word.len();
                }
            }
        }
        trie
    }

    pub fn query(&self, s: &str) -> usize {
        let mut node = &self.root;
        for b in s.bytes().rev() {
            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child,
                None => break,
            }
        }
        node.best_index
    }
}

pub struct Solution;

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        return Self::v1(words_container, words_query);
    }

    fn v1(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let trie = SuffixTrie::from_container(&words_container);

        words_query
            .iter()
            .map(|query| trie.query(query) as i32)
            .collect()
    }
}

pub fn generate_worst_case(
    container_count: usize,
    query_count: usize,
    string_len: usize,
) -> (Vec<String>, Vec<String>) {
    let s = "a".repeat(string_len);
    let container = vec![s.clone(); container_count];
    let query = vec![s; query_count];
    (container, query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words_container = ["abcd", "bcd", "xbcd"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let words_query = ["cd", "bcd", "xyz"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let result = vec![1, 1, 1];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            result
        );
    }

    #[test]
    fn example_2() {
        let words_container = ["abcdefgh", "poiuygh", "ghghgh", "ghghgh"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let words_query = ["gh", "acbfgh", "acbfegh"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let result = vec![2, 0, 2];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            result
        );
    }

    #[test]
    fn example_3() {
        let words_container = ["a", "b"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let words_query = ["a", "b"]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();
        let result = vec![0, 1];
        assert_eq!(
            Solution::string_indices(words_container, words_query),
            result
        );
    }

    #[test]
    fn worst_case_small() {
        let (container, query) = generate_worst_case(10_000, 10_000, 100);
        let result = Solution::string_indices(container, query);
        assert_eq!(result.len(), 10_000);
    }
}
