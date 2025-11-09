/*
 * [3402] Minimum Operations to Make Columns Strictly Increasing
 */

// @lc code=start
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for col in 0..grid[0].len() {
            let mut pre = -1;
            for row in 0..grid.len() {
                let val = grid[row][col];
                ans += 0.max(pre - val + 1);
                pre = val.max(pre + 1);
            }
        }
        ans as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[3,2],[1,3],[3,4],[0,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[3,2,1],[2,1,0],[1,2,3]]\n
// @lcpr case=end

 */

