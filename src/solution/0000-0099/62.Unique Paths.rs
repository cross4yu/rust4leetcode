/*
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut ans = 0;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 0..m {
            for j in 0..n {
                dp[j + 1] += dp[j];
            }
        }
        dp[n]
    }
}
// @lc code=end



/*
// @lcpr case=start
// 3\n7\n
// @lcpr case=end

// @lcpr case=start
// 3\n2\n
// @lcpr case=end

 */

