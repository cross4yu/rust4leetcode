/*
 * [1512] Number of Good Pairs
 */

// @lc code=start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut result = 0;
        for &num in &nums {
            let entry = cnt.entry(num).or_insert(0);
            result += *entry;
            *entry += 1;
        }
        result as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,1,1,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

 */

