/*
 * [3693] Climbing Stairs II
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let len = costs.len();
        let mut dp0 = 0;
        let mut dp1 = 0;
        let mut dp2 = 0;
        for i in 0..len {
            let temp = (dp0 + 9).min((dp1 + 4).min(dp2 + 1)) + costs[i];
            dp0 = dp1;
            dp1 = dp2;
            dp2 = temp;
        }
        dp2
    }
}
// @lc code=end



/*
// @lcpr case=start
// 4\n[1,2,3,4]\n
// @lcpr case=end

// @lcpr case=start
// 4\n[5,1,6,2]\n
// @lcpr case=end

// @lcpr case=start
// 3\n[9,8,3]\n
// @lcpr case=end

 */

