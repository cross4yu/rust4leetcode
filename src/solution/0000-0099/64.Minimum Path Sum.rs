/*
 * [64] Minimum Path Sum
 */

// @lc code=start
impl Solution {

    // dp[i+1][j+1] = min(dp[i+1][j], dp[i][j+1]) + g[i][j];
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[1] = 0;
        for i in 0..grid.len() {
            for j in 0..n {
                dp[j + 1] = dp[j + 1].min(dp[j]) + grid[i][j];
            }
        }
        dp[n]
    }

    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for j in 1..n {
            grid[0][j] += grid[0][j - 1];
        }
        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
            for j in 1..n {
                grid[i][j] += grid[i][j - 1].min(grid[i - 1][j]);
            }
        }
        grid[m - 1][n - 1]
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[1,3,1],[1,5,1],[4,2,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,2,3],[4,5,6]]\n
// @lcpr case=end

 */

