/*
 * @lc app=leetcode id=3634 lang=rust
 * @lcpr version=30203
 *
 * [3634] Minimum Removals to Balance Array
 */

// @lc code=start
impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut windows = 0;
        for (right, &num) in nums.iter().enumerate() {
            while (nums[left] as i64) * (k as i64) < (num as i64) {
                left += 1;
            }
            windows = windows.max(right - left + 1);
        }

        nums.len() - windows
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,1,5]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,6,2,9]\n3\n
// @lcpr case=end

// @lcpr case=start
// [4,6]\n2\n
// @lcpr case=end

 */

