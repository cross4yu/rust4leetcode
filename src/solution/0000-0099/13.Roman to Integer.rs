/*
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let dict = HashMap::from([
            ('M', 1000),
            ('D', 500),
            ('C', 100),
            ('L', 50),
            ('X', 10),
            ('V', 5),
            ('I', 1),
        ]);
        let mut res = 0;
        let mut pre = 0;
        for v in s.chars().rev() {
            let cur = if let Some(&val) = dict.get(&v) {
                val
            } else {
                return 0;
            };
            if cur >= pre {
                res += cur;
                pre = cur;
            } else {
                res -= cur;
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// "III"\n
// @lcpr case=end

// @lcpr case=start
// "LVIII"\n
// @lcpr case=end

// @lcpr case=start
// "MCMXCIV"\n
// @lcpr case=end

 */

