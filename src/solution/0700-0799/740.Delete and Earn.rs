/*
 * [740] Delete and Earn
 */

// @lc code=start
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max_num = nums.iter().max().unwrap();
        let mut cnt = vec![0; *max_num as usize + 1];
        for &num in &nums {
            cnt[num as usize] += num;
        }
        Self::rob(cnt)
    }

    fn rob(nums: Vec<i32>) -> i32 {
        let mut dp0 = 0;
        let mut dp1 = 0;
        for &num in &nums {
            let temp = dp1.max(dp0 + num);
            dp0 = dp1;
            dp1 = temp;
        }
        dp1
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,4,2]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,3,3,3,4]\n
// @lcpr case=end

 */

