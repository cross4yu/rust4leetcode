/*
 * [213] House Robber II
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }
        // i32::max(Self::rob_n(&nums[2..nums.len() - 1]) + nums[0], Self::rob_n(&nums[1..nums.len()]))
        // not first one && not last one
        i32::max(Self::rob_n(&nums[1..]), Self::rob_n(&nums[..nums.len() - 1]))
    }

    pub fn rob_n(nums: &[i32]) -> i32 {
        let mut dp0 = 0;
        let mut dp1 = 0;
        for &i in nums {
            let temp = dp1.max(dp0 + i);
            dp0 = dp1;
            dp1 = temp;
        }
        dp1
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,3,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n
// @lcpr case=end

 */

