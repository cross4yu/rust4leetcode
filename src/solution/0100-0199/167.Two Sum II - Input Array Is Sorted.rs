/*
 * @lc app=leetcode id=167 lang=rust
 * @lcpr version=30203
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return vec![-1, -1];
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,7,11,15]\n9\n
// @lcpr case=end

// @lcpr case=start
// [2,3,4]\n6\n
// @lcpr case=end

// @lcpr case=start
// [-1,0]\n-1\n
// @lcpr case=end

 */

