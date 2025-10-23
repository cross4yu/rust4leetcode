/*
 * [2684] Maximum Number of Moves in a Grid
 */

// @lc code=start
impl Solution {
    pub fn max_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, j: usize, ans: &mut usize, grid: &mut Vec<Vec<i32>>) {
            *ans = (*ans).max(j);
            if *ans == grid[0].len() - 1 {
                return;
            }
            for k in i.saturating_sub(1)..grid.len().min(i + 2) {
                if grid[k][j + 1] > grid[i][j] {
                    dfs(k, j + 1, ans, grid);
                }
            }
            grid[i][j] = 0;
        }
        let mut ans = 0;
        for i in 0..grid.len() {
            dfs(i, 0 , &mut ans, &mut grid);
        }
        ans as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]\n
// @lcpr case=end

// @lcpr case=start
// [[3,2,4],[2,1,9],[1,1,7]]\n
// @lcpr case=end

 */

