/*
 * @lc app=leetcode id=274 lang=rust
 * @lcpr version=30203
 *
 * [274] H-Index
 */

// @lc code=start
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let len = citations.len() as i32;
        for (i, &c) in citations.iter().enumerate() {
            if c >= len - i as i32 {
                return len - i as i32;
            }
        }
        0
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,0,6,1,5]\n
// @lcpr case=end

// @lcpr case=start
// [1,3,1]\n
// @lcpr case=end

 */

