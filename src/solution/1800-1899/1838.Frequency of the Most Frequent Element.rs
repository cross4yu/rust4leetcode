/*
 * [1838] Frequency of the Most Frequent Element
 */

// @lc code=start
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let mut left = 0;
        let mut sum = 0i64;
        for right in 0..nums.len() {
            sum += nums[right] as i64;
            while (right + 1 - left) as i64 * nums[right] as i64 - sum > k as i64 {
                sum -= nums[left] as i64;
                left += 1;
            }
            res = res.max(right + 1 - left);
        }
        res as i32
    }
}


// @lc code=end



/*
// @lcpr case=start
// [1,2,4]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,4,8,13]\n5\n
// @lcpr case=end

// @lcpr case=start
// [3,9,6]\n2\n
// @lcpr case=end

 */

