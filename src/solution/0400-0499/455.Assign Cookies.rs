/*
 * [455] Assign Cookies
 */

// @lc code=start
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        s.sort_unstable();
        g.sort_unstable();
        let mut ans = 0;
        let mut i = g.len() as i32 - 1;
        let mut j = s.len() as i32 - 1;
        while i >= 0 && j >= 0 {
            if g[i as usize] <= s[j as usize] {
                ans += 1;
                i -= 1;
                j -= 1;
            } else {
                i -= 1;
            }
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3]\n[1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2]\n[1,2,3]\n
// @lcpr case=end

 */

