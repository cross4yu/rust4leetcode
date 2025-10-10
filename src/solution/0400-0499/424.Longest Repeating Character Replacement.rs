/*
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut res = 0;
        let s: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut max = 0;
        let mut cnt = std::collections::HashMap::new();
        for (right, &c) in s.iter().enumerate() {
            *cnt.entry(c as usize - 'A' as usize).or_insert(0) += 1;
            max = max.max(*cnt.get(&(c as usize - 'A' as usize)).unwrap());
            if (right + 1 - left) as i32 > k + max {
                cnt.entry(s[left] as usize - 'A' as usize).and_modify(|v| *v -= 1);
                left += 1;
            }
            res = res.max((right - left + 1) as i32);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "ABAB"\n2\n
// @lcpr case=end

// @lcpr case=start
// "AABABBA"\n1\n
// @lcpr case=end

 */

