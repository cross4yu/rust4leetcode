/*
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        let mut left = 1;
        for i in 0..n {
            result[i] = left;
            left *= nums[i];
        }
        let mut right = 1;
        for i in (0..n).rev() {
            result[i] *= right;
            right *= nums[i];
        }
        result
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

// @lcpr case=start
// [-1,1,0,-3,3]\n
// @lcpr case=end

 */

