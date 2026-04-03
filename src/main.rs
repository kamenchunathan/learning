use learning::solutions::s3661::Solution;

fn main() {
    let result1 = Solution::max_walls(vec![4], vec![3], vec![1, 10]);
    println!("Example 1: {}", result1);

    let result2 = Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]);
    println!("Example 2: {}", result2);

    let result3 = Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]);
    println!("Example 3: {}", result3);
}
