// 3161. Block Placement Queries
// There exists an infinite number line, with its origin at 0 and extending towards the positive x-axis.
//
// You are given a 2D array queries, which contains two types of queries:
//
// For a query of type 1, queries[i] = [1, x]. Build an obstacle at distance x from the origin. It is guaranteed that there is no obstacle at distance x when the query is asked.
// For a query of type 2, queries[i] = [2, x, sz]. Check if it is possible to place a block of size sz anywhere in the range [0, x] on the line, such that the block entirely lies in the range [0, x]. A block cannot be placed if it intersects with any obstacle, but it may touch it. Note that you do not actually place the block. Queries are separate.
// Return a boolean array results, where results[i] is true if you can place the block specified in the ith query of type 2, and false otherwise.
//
//
//
// Example 1:
//
// Input: queries = [[1,2],[2,3,3],[2,3,1],[2,2,2]]
//
// Output: [false,true,true]
//
// Explanation:
//
//
//
// For query 0, place an obstacle at x = 2. A block of size at most 2 can be placed before x = 3.
//
// Example 2:
//
// Input: queries = [[1,7],[2,7,6],[1,2],[2,7,5],[2,7,6]]
//
// Output: [true,true,false]
//
// Explanation:
//
//
//
// Place an obstacle at x = 7 for query 0. A block of size at most 7 can be placed before x = 7.
// Place an obstacle at x = 2 for query 2. Now, a block of size at most 5 can be placed before x = 7, and a block of size at most 2 before x = 2.
//
//
// Constraints:
//
// 1 <= queries.length <= 15 * 104
// 2 <= queries[i].length <= 3
// 1 <= queries[i][0] <= 2
// 1 <= x, sz <= min(5 * 104, 3 * queries.length)
// The input is generated such that for queries of type 1, no obstacle exists at distance x when the query is asked.
// The input is generated such that there is at least one query of type 2.

use std::{collections::BTreeMap, convert::identity, ops::Range};

#[derive(Debug)]
struct SegmentTreeNode {
    data: u32,
    left: Option<Box<SegmentTreeNode>>,
    right: Option<Box<SegmentTreeNode>>,
}

#[derive(Debug)]
struct SegmentTree {
    root: Option<Box<SegmentTreeNode>>,
}

impl SegmentTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert_split(&mut self, pivot: u32) {
        let mut node = &mut self.root;
        loop {
            match node {
                Some(n) => {
                    // Do nothing it's already split
                    if pivot == n.data {
                        break;
                    }

                    if pivot < n.data {
                        node = &mut n.left;
                    } else {
                        node = &mut n.right;
                    }

                    continue;
                }
                None => {
                    *node = Some(Box::new(SegmentTreeNode {
                        data: pivot,
                        left: None,
                        right: None,
                    }));

                    break;
                }
            }
        }
    }

    fn query(&self, qr: u32, sz: u32) -> bool {
        let mut nodes = vec![(&self.root, (0..u32::MAX))];

        loop {
            let Some((node, range)) = nodes.pop() else {
                return false;
            };

            match node {
                // Split case
                Some(node) => {
                    // recurse only into segments that are large enough to contain
                    // the block
                    let right = u32::min(node.data, qr);
                    if right.saturating_sub(range.start) >= sz {
                        nodes.push((&node.left, range.start..right));
                    }

                    let right = u32::min(range.end, qr);
                    if right.saturating_sub(node.data) >= sz {
                        nodes.push((&node.right, node.data..right));
                    }
                }
                None => {
                    if u32::min(range.end, qr).saturating_sub(range.start) >= sz {
                        // Early exit
                        return true;
                    }
                }
            }
        }
    }
}

fn contains_range<T: PartialOrd>(outer: &Range<T>, inner: &Range<T>) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

struct Solution;

impl Solution {
    pub fn get_results_segtree(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut segtree = SegmentTree::new();
        let mut results = Vec::with_capacity(queries.len());
        for query in queries {
            match query[0] {
                1 => segtree.insert_split(query[1] as u32),
                2 => results.push(segtree.query(query[1] as u32, query[2] as u32)),
                _ => {}
            }
        }
        results
    }

    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut obstacles = BTreeMap::new();
        let mut results = Vec::with_capacity(queries.len());

        obstacles.insert(0, ());

        for query in queries {
            match query[0] {
                1 => {
                    obstacles.insert(query[1], ());
                }
                2 => {
                    let upper_bound = query[1];
                    let size = query[2];

                    let transient_obstacle = obstacles.insert(upper_bound, ()).is_none();
                    let a = obstacles.range(0..=upper_bound).collect::<Vec<_>>();

                    results.push(
                        a.windows(2)
                            .map(|w| {
                                let a = w[0].0;
                                let b = w[1].0;

                                (*b - *a) >= size
                            })
                            .any(identity),
                    );

                    if transient_obstacle {
                        obstacles.remove(&upper_bound);
                    }
                }
                _ => {}
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let queries = vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]];
        assert_eq!(Solution::get_results(queries), vec![false, true, true])
    }

    #[test]
    fn example_2() {
        let queries = vec![
            vec![1, 7],
            vec![2, 7, 6],
            vec![1, 2],
            vec![2, 7, 5],
            vec![2, 7, 6],
        ];
        assert_eq!(Solution::get_results(queries), vec![true, true, false])
    }

    #[test]
    fn no_obstacle() {
        let queries = vec![vec![2, 1, 1]];
        assert_eq!(Solution::get_results(queries), vec![true]);

        let segs = SegmentTree::new();
        assert_eq!(segs.query(1, 1), true);
    }

    #[test]
    fn wo() {
        let mut segs = SegmentTree::new();
        segs.insert_split(3);
        assert_eq!(segs.query(1, 2), false);
        assert_eq!(segs.query(1, 1), true);
        assert_eq!(segs.query(5, 4), false);
        assert_eq!(segs.query(10, 5), true);
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    fn random_query_strategy(max_coord: i32) -> impl Strategy<Value = Vec<Vec<i32>>> {
        (
            proptest::collection::vec(1i32..=max_coord, 0..10),
            proptest::collection::vec((1i32..=max_coord, 1i32..=max_coord), 1..20),
        )
            .prop_map(|(obs, queries_t2)| {
                let mut queries: Vec<Vec<i32>> = Vec::new();
                for o in obs {
                    queries.push(vec![1, o]);
                }
                for (x, sz) in queries_t2 {
                    queries.push(vec![2, x, sz]);
                }
                queries
            })
            .prop_shuffle()
    }

    proptest! {
        #[test]
        fn segtree_matches_btreemap(
            queries in random_query_strategy(100)
        ) {
            let expected = Solution::get_results(queries.clone());
            let actual = Solution::get_results_segtree(queries);
            prop_assert_eq!(expected, actual);
        }
    }
}
