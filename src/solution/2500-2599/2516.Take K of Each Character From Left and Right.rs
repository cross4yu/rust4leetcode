/*
 * [2516] Take K of Each Character From Left and Right
 */

// @lc code=start
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let sc = s.chars().collect::<Vec<char>>();
        let mut cnt = vec![0; 3];
        let len = sc.len();
        let mut res = len;
        for c in sc.iter() {
            cnt[*c as usize - 'a' as usize] += 1;
        }
        if cnt[0] < k || cnt[1] < k || cnt[2] < k {
            return -1;
        }
        let mut left = 0;
        for right in 0..len {
            cnt[sc[right] as usize - 'a' as usize] -= 1;
            while cnt[0] < k || cnt[1] < k || cnt[2] < k {
                cnt[sc[left] as usize - 'a' as usize] += 1;
                left += 1;
            }
            if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
                res = res.min(len - (right + 1 - left));
            }
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// "aabaaaacaabc"\n2\n
// @lcpr case=end

// @lcpr case=start
// "a"\n1\n
// @lcpr case=end

 */

