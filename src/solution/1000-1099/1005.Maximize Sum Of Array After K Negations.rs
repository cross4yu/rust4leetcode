/*
 * [1005] Maximize Sum Of Array After K Negations
 */

// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut k = k;
        let mut ans = 0;
        for i in 0..nums.len() {
            if (nums[i] < 0 && k > 0) {
                nums[i] = -1 * nums[i];
                k -= 1;
            }
            ans += nums[i];
        }
        nums.sort_unstable();
        return if k % 2 == 0 {
            ans
        } else {
            ans - (2 * nums[0])
        }
    }
}
// @lc code=end



/*
// @lcpr case=start
// [4,2,3]\n1\n
// @lcpr case=end

// @lcpr case=start
// [3,-1,0,2]\n3\n
// @lcpr case=end

// @lcpr case=start
// [2,-3,-1,5,-4]\n2\n
// @lcpr case=end

 */

