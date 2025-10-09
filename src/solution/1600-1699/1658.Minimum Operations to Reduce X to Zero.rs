/*
 * [1658] Minimum Operations to Reduce X to Zero
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;
        if target < 0 {
            return -1;
        }
        let mut res = -1;
        let mut left = 0;
        let mut sum = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum > target {
                sum -= nums[left];
                left += 1;
            }
            if sum == target {
                res = res.max((right + 1 - left) as i32);
            }
        }
        if res < 0 { -1 }  else {nums.len() as i32 - res}
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,1,4,2,3]\n5\n
// @lcpr case=end

// @lcpr case=start
// [5,6,7,8,9]\n4\n
// @lcpr case=end

// @lcpr case=start
// [3,2,20,1,1,3]\n10\n
// @lcpr case=end

 */

