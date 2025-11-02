/*
 * [1833] Maximum Ice Cream Bars
 */

// @lc code=start
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        let mut ans = 0;
        let mut retained = coins;
        for &cost in &costs {
            if retained >= cost {
                retained -= cost;
                ans += 1;
            } else {
                break;
            }
        }
        ans as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,2,4,1]\n7\n
// @lcpr case=end

// @lcpr case=start
// [10,6,8,7,7,8]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,6,3,1,2,5]\n20\n
// @lcpr case=end

 */

