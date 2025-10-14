/*
 * [746] Min Cost Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp0 = 0;
        let mut dp1 = 0;
        for i in 1..len {
            let temp = (cost[i] + dp1).min(cost[i - 1] + dp0);
            dp0 = dp1;
            dp1 = temp;
        }
        dp1
    }
}
// @lc code=end



/*
// @lcpr case=start
// [10,15,20]\n
// @lcpr case=end

// @lcpr case=start
// [1,100,1,1,1,100,1,1,100,1]\n
// @lcpr case=end

 */

