/*
 * [1208] Get Equal Substrings Within Budget
 */

// @lc code=start
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let diffs:Vec<i32> = s.chars().zip(t.chars()).map(|(c1, c2)| {(c1 as i32 - c2 as i32).abs()}).collect::<Vec<_>>();
        let mut cost = 0;
        let mut left = 0;
        let mut sum = 0;
        for (right, &diff) in diffs.iter().enumerate() {
            sum += diff;
            while sum > max_cost {
                sum -= diffs[left];
                left += 1;
            }
            cost = cost.max(right + 1 -left);
        }
        cost as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "abcd"\n"bcdf"\n3\n
// @lcpr case=end

// @lcpr case=start
// "abcd"\n"cdef"\n3\n
// @lcpr case=end

// @lcpr case=start
// "abcd"\n"acde"\n0\n
// @lcpr case=end

 */

