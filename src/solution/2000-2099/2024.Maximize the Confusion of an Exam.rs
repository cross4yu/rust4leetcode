/*
 * [2024] Maximize the Confusion of an Exam
 */

// @lc code=start
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let s = answer_key.as_bytes();
        let mut res = 0;
        let mut left = 0;
        let mut cnt = [0, 0];
        for (right, &key) in s.iter().enumerate() {
            cnt[(key >> 1 & 1) as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[(s[left] >> 1 & 1) as usize] -= 1;
                left += 1;
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "TTFF"\n2\n
// @lcpr case=end

// @lcpr case=start
// "TFFT"\n1\n
// @lcpr case=end

// @lcpr case=start
// "TTFTTFTT"\n1\n
// @lcpr case=end

 */

