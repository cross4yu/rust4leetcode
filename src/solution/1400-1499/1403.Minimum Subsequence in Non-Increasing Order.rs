/*
 * [1403] Minimum Subsequence in Non-Increasing Order
 */

// @lc code=start
impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let sum = nums.iter().sum::<i32>();
        let mut cur = 0;
        let mut ans = Vec::new();
        for i in (0..nums.len()).rev() {
            cur += nums[i];
            ans.push(nums[i]);
            if cur > sum - cur {
                break;
            }
        }
        ans
    }
}
// @lc code=end



/*
// @lcpr case=start
// [4,3,10,9,8]\n
// @lcpr case=end

// @lcpr case=start
// [4,4,7,6,7]\n
// @lcpr case=end

 */

