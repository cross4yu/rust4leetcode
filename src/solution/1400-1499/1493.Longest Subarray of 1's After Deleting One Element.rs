/*
 * [1493] Longest Subarray of 1's After Deleting One Element
 */

// @lc code=start
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut last_zero = -1;
        for (i, &num) in nums.iter().enumerate() {
            if num == 0 {
                left = last_zero + 1;
                last_zero = i as i32;
            }
            res = res.max(i - left as usize);
        }
        res as i32
    }

    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            cnt += 1 - num;
            while cnt > 1 {
                cnt -= 1 - nums[left];
                left += 1;
            }
            res = res.max(right - left);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,1,0,1]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,1,1,0,1,1,0,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1]\n
// @lcpr case=end

 */

