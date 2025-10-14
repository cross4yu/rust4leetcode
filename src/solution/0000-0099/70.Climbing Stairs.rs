/*
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp0 = 1;
        let mut dp1 = 1;
        for i in 0..n - 1 {
            let temp = dp0 + dp1;
            dp0 = dp1;
            dp1 = temp;
        }
        dp1
    }
}
// @lc code=end



/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 3\n
// @lcpr case=end

 */

