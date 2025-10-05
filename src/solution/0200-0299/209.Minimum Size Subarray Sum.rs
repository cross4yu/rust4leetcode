/*
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = len + 1;
        let mut sum = 0;
        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                res = res.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if res > len { 0 } else { res as i32 }
    }
}
// @lc code=end



/*
// @lcpr case=start
// 7\n[2,3,1,2,4,3]\n
// @lcpr case=end

// @lcpr case=start
// 4\n[1,4,4]\n
// @lcpr case=end

// @lcpr case=start
// 11\n[1,1,1,1,1,1,1,1]\n
// @lcpr case=end

 */

