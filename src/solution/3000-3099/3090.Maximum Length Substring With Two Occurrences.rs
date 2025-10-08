/*
 * [3090] Maximum Length Substring With Two Occurrences
 */

// @lc code=start
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        let mut left = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            let c = chars[i];
            *cnt.entry(c).or_insert(0) += 1;
            while *cnt.get(&c).unwrap() > 2 {
                cnt.entry(chars[left]).and_modify(|x| *x -= 1);
                left += 1;
            }
            res = res.max(i + 1 - left);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "bcbbbcba"\n
// @lcpr case=end

// @lcpr case=start
// "aaaa"\n
// @lcpr case=end

 */

