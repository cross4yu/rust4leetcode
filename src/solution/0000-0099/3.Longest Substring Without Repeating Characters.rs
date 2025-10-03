/*
 * [3] Longest Substring Without Repeating Characters
 */
use std::collections::HashSet;
use std::collections::HashMap;

// @lc code=start
impl Solution {
    // set
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut set = HashSet::new();
        let mut left = 0;
        let mut ans = 0;
        for (i, &c) in s.iter().enumerate() {
            if set.contains(&c) {
                while s[left] != c {
                    set.remove(&s[left]);
                    left += 1;
                }
                left+=1; // repeat item
            }
            set.insert(c);
            ans = ans.max(i - left + 1);
        }
        ans as i32
    }

    // hash
    pub fn length_of_longest_substring2(s: String) -> i32 {
        let s = s.as_bytes();
        let mut hm = HashMap::new();
        let mut left = 0;
        let mut ans = 0;
        for (i, &c) in s.iter().enumerate() {
            if let Some(v) = hm.get(&c) {
                left = left.max(v+1);
            }
            hm.insert(c, i);
            ans = ans.max(i - left + 1);
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "abcabcbb"\n
// @lcpr case=end

// @lcpr case=start
// "bbbbb"\n
// @lcpr case=end

// @lcpr case=start
// "pwwkew"\n
// @lcpr case=end

 */

