/*
 * [2815] Max Pair Sum in an Array
 */

// @lc code=start
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut cnt = [i32::MIN; 10];
        for &num in &nums {
            let mut temp = num;
            let mut idx = 0;
            while temp > 0 {
                idx = idx.max(temp as usize % 10);
                temp /= 10;
            }
            if cnt[idx] > 0 {
                ans = ans.max(cnt[idx] + num);
            }
            cnt[idx] = cnt[idx].max(num);
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [112,131,411]\n
// @lcpr case=end

// @lcpr case=start
// [2536,1613,3366,162]\n
// @lcpr case=end

// @lcpr case=start
// [51,71,17,24,42]\n
// @lcpr case=end

 */

