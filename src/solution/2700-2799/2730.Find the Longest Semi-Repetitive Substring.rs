/*
 * [2730] Find the Longest Semi-Repetitive Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut res = 0;
        let mut left = 0;
        let mut same = 0;
        for right in 1..s.len() {
            if s[right - 1] == s[right] {
                same += 1;
            }
            if same > 1 {
                left += 1;
                while s[left - 1] != s[left] {
                    left += 1;
                }
                same -= 1;
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "52233"\n
// @lcpr case=end

// @lcpr case=start
// "5494"\n
// @lcpr case=end

// @lcpr case=start
// "1111111"\n
// @lcpr case=end

// @lcpr case=start
// "0001"\n
// @lcpr case=end

// @lcpr case=start
// "0"\n
// @lcpr case=end

 */

