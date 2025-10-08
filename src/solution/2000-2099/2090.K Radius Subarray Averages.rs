/*
 * @lc app=leetcode id=2090 lang=rust
 * @lcpr version=30203
 *
 * [2090] K Radius Subarray Averages
 */

// @lc code=start
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res  = vec![-1;nums.len()];
        let windows = k * 2 + 1;
        let k = k as usize;
        let mut sum: i64 = 0;
        for (i, &v) in nums.iter().enumerate() {
            sum += v as i64;
            if i < 2 * k {
                continue;
            }
            res[i - k] = (sum / (k as i64 * 2 + 1)) as i32;
            sum -= nums[i - 2 * k] as i64;
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [7,4,3,9,1,8,5,2,6]\n3\n
// @lcpr case=end

// @lcpr case=start
// [100000]\n0\n
// @lcpr case=end

// @lcpr case=start
// [8]\n100000\n
// @lcpr case=end

 */

