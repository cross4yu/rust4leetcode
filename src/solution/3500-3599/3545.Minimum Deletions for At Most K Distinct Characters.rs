/*
 * [3545] Minimum Deletions for At Most K Distinct Characters
 */

// @lc code=start
impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut cnt = vec![0; 26];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        cnt.sort_unstable();
        let mut ans = 0;
        for i in 0..(26 - k as usize) {
            ans += cnt[i];
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "abc"\n2\n
// @lcpr case=end

// @lcpr case=start
// "aabb"\n2\n
// @lcpr case=end

// @lcpr case=start
// "yyyzz"\n1\n
// @lcpr case=end

 */

