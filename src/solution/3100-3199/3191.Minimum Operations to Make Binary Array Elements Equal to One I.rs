/*
 * [3191] Minimum Operations to Make Binary Array Elements Equal to One I
 */

// @lc code=start
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                ans += 1;
            }
        }
        if nums[nums.len() - 1] != 0 && nums[nums.len() - 2] != 0 { ans } else { -1 }
    }
}
// @lc code=end



/*
// @lcpr case=start
// [0,1,1,1,0,0]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,1,1]\n
// @lcpr case=end

 */

