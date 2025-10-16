/*
 * [198] House Robber
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp0 = 0;
        let mut dp1 = 0;
        for &num in &nums {
            let temp = dp1.max(dp0 + num);
            dp0 = dp1;
            dp1 = temp;
        }
        dp1
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,1]\n
// @lcpr case=end

// @lcpr case=start
// [2,7,9,3,1]\n
// @lcpr case=end

 */

