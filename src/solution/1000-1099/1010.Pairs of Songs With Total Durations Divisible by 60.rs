/*
 * [1010] Pairs of Songs With Total Durations Divisible by 60
 */

// @lc code=start
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cnt = [0; 60];
        for t in &time {
            res += cnt[(60 -  (*t as usize) % 60) % 60];
            cnt[(*t as usize) % 60] += 1;
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [30,20,150,100,40]\n
// @lcpr case=end

// @lcpr case=start
// [60,60,60]\n
// @lcpr case=end

 */

