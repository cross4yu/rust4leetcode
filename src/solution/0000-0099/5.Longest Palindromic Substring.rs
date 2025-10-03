/*
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    // DP
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        if len < 2 {
            return String::from_iter(s)
        }
        let mut dp = vec![vec![false; len]; len];
        for i in 0..len {
            dp[i][i] = true;
        }
        let mut start = 0;
        let mut max_len = 1;
        for sub_len in 2..=len {
            for i in 0..len {
                let j = i + sub_len - 1;
                if j >= len {
                    break;
                }
                if s[j] == s[i] {
                    if sub_len < 4 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                } else {
                    dp[i][j] = false;
                }
                if dp[i][j] && sub_len > max_len {
                    start = i;
                    max_len = sub_len;
                }
            }
        }
        s[start..start + max_len].iter().collect()
    }

    // Centralization
    pub fn longest_palindrome_center(s: String) -> String {
        if s.len() < 2 {
            return s
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut start = 0;
        let mut end = 0;
        for i in 0..s.len() {
            let (start1, end1) = Self::center_check(&s, i as i32, i as i32);
            let (start2, end2) = Self::center_check(&s, i as i32, (i + 1) as i32);
            if end1 - start1 > end2 - start2 {
                if end1 - start1 > end - start {
                    end = end1;
                    start = start1;
                }
            } else {
                if end2 - start2 > end - start {
                    end = end2;
                    start = start2;
                }
            }
        }
        s[start..=end].iter().collect()
    }

    fn center_check(s: &Vec<char>, mut left: i32, mut right: i32) -> (usize, usize) {
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        let start = (left + 1) as usize;
        let end = (right - 1) as usize;
        if start > end {
            return (start, start);
        }
        (start, end)
    }

    // Manacher

}
// @lc code=end



/*
// @lcpr case=start
// "babad"\n
// @lcpr case=end

// @lcpr case=start
// "cbbd"\n
// @lcpr case=end

 */

