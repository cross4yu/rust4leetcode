/*
 * [643] Maximum Average Subarray I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut res = i32::MIN;
        let mut sum = 0;
        for (i, &v) in nums.iter().enumerate() {
            sum += v;
            if i + 1 < k {
                continue;
            }
            res = res.max(sum);
            sum -= nums[i + 1 - k];
        }
        res as f64 / k as f64
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,12,-5,-6,50,3]\n4\n
// @lcpr case=end

// @lcpr case=start
// [5]\n1\n
// @lcpr case=end

 */

