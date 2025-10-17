/*
 * [2320] Count Number of Ways to Place Houses
 */

// @lc code=start
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        if n == 1 {
            return 4;
        }
        let mut dp : Vec<i64> = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..=n as usize {
            dp[i] = (dp[i - 1] + dp[i - 2]) % MOD;
        }
        ((dp[n as usize] * dp[n as usize]) % MOD) as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// 1\n
// @lcpr case=end

// @lcpr case=start
// 2\n
// @lcpr case=end

 */

