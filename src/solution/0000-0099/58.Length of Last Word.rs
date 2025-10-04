/*
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut n = s.len();
        if n <= 0 {
            return 0;
        }
        n -= 1;
        while s[n] == ' ' {
            n -= 1;
        }
        let mut j = n as i32 - 1;
        while j >= 0 && s[j as usize] != ' ' {
            j -= 1;
        }
        n as i32 - j
    }
}
// @lc code=end



/*
// @lcpr case=start
// "Hello World"\n
// @lcpr case=end

// @lcpr case=start
// "   fly me   to   the moon  "\n
// @lcpr case=end

// @lcpr case=start
// "luffy is still joyboy"\n
// @lcpr case=end

 */

