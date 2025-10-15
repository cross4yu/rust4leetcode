/*
 * [377] Combination Sum IV
 */

// @lc code=start
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in &nums {
                let num = num as usize;
                if num <= i {
                    dp[i] += dp[i - num];
                }
            }
        }
        dp[target]
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let nums = nums.into_iter()
            .filter(|&x| x as usize <= target)
            .map(|x| x as usize)
            .collect::<Vec<_>>();
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0i32; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in &nums {
                if num <= i {
                    dp[i] = dp[i].saturating_add(dp[i - num]);
                }
            }
        }
        dp[target]
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3]\n4\n
// @lcpr case=end

// @lcpr case=start
// [9]\n3\n
// @lcpr case=end

 */

