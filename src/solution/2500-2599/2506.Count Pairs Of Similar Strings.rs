/*
 * [2506] Count Pairs Of Similar Strings
 */

// @lc code=start
impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        for word in &words {
            let mut mask = 0;
            for b in word.as_bytes() {
                mask |= 1 << (b - b'a');
            }
            let entry = cnt.entry(mask).or_insert(0);
            res += *entry;
            *entry += 1;
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// ["aba","aabb","abcd","bac","aabc"]\n
// @lcpr case=end

// @lcpr case=start
// ["aabb","ab","ba"]\n
// @lcpr case=end

// @lcpr case=start
// ["nba","cba","dba"]\n
// @lcpr case=end

 */

