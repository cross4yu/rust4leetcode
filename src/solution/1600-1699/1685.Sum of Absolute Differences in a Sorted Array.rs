/*
 * [1685] Sum of Absolute Differences in a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }
        let mut ans = vec![0; nums.len()];
        for i in 0..nums.len() {
            ans[i] = (sum[n] - sum[i]) - (sum[i] - sum[0]) + nums[i] * (i as i32 - (n - i) as i32);
        }
        ans
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,3,5]\n
// @lcpr case=end

// @lcpr case=start
// [1,4,6,8,10]\n
// @lcpr case=end

 */

