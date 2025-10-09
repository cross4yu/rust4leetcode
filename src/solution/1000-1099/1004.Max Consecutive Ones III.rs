/*
 * [1004] Max Consecutive Ones III
 */

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut cnt = 0;
        for (right, &num) in nums.iter().enumerate() {
            cnt += 1 - num;
            while cnt > k {
                cnt -= 1 - nums[left];
                left += 1;
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,1,1,0,0,0,1,1,1,1,0]\n2\n
// @lcpr case=end

// @lcpr case=start
// [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1]\n3\n
// @lcpr case=end

 */

