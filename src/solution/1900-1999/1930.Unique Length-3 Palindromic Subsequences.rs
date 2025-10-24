/*
 * [1930] Unique Length-3 Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let (mut pre, mut suf) = (vec![0; n], vec![0; n]);
        for i in 0..n {
            pre[i] = if i == 0 {0} else {pre[i-1]} | 1 << s[i] - b'a';
        }
        for i in (0..n).rev() {
            suf[i] = if i == n - 1 {0} else {suf[i+1]} | 1 << s[i] - b'a';
        }
        let mut ans = vec![0i32; 26];
        for i in 1..n-1 {
            ans[(s[i] - b'a') as usize] |= pre[i - 1] & suf[i + 1];
        }
        // ans.into_iter().fold(0, |sum, x| sum + x.count_ones()) as _
        ans.iter().map(|&mask| mask.count_ones() as i32).sum()
    }
}
// @lc code=end



/*
// @lcpr case=start
// "aabca"\n
// @lcpr case=end

// @lcpr case=start
// "adc"\n
// @lcpr case=end

// @lcpr case=start
// "bbcbaba"\n
// @lcpr case=end

 */

