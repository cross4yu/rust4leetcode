/*
 * [2779] Maximum Beauty of an Array After Applying Operation
 */

// @lc code=start
impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut res = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            while nums[right] - nums[left] > k * 2 {
                left += 1;
            }
            res = res.max(right + 1 - left);
        }
        res as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [4,6,1,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1,1]\n10\n
// @lcpr case=end

 */

