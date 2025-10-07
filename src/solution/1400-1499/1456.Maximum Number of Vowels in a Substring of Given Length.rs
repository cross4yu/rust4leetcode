/*
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut res = 0;
        let mut cnt = 0;
        let s = s.chars().collect::<Vec<char>>();
        let windows = k as usize;
        for (i, &v) in s.iter().enumerate() {
            if vowels.contains(&v) {
                cnt += 1;
            }
            if i + 1 < windows {
                continue;
            }
            res = res.max(cnt);
            if res == k {
                return k;
            }
            let out = s[i + 1 - windows];
            if vowels.contains(&out) {
                cnt -= 1;
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// "abciiidef"\n3\n
// @lcpr case=end

// @lcpr case=start
// "aeiou"\n2\n
// @lcpr case=end

// @lcpr case=start
// "leetcode"\n3\n
// @lcpr case=end

 */

