/*
 * [2379] Minimum Recolors to Get K Consecutive Black Blocks
 */

// @lc code=start
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let s = blocks.chars().collect::<Vec<char>>();
        let k = k as usize;
        let mut cnt = 0;
        let mut res = i32::MAX;
        for (i, &v) in s.iter().enumerate() {
            if v == 'W' {
                cnt += 1;
            }
            if i + 1 < k {
                continue;
            }
            res = res.min(cnt);
            if s[i + 1 - k] == 'W' {
                cnt -= 1;
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// "WBBWWBBWBW"\n7\n
// @lcpr case=end

// @lcpr case=start
// "WBWBBBW"\n2\n
// @lcpr case=end

 */

