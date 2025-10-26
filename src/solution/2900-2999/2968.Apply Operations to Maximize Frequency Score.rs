/*
 * [2968] Apply Operations to Maximize Frequency Score
 */

// @lc code=start
impl Solution {
    // prefix + windows
    pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut sum = vec![0i64; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let mut ans = 0;
        let mut left = 0;
        fn distance(sum: &Vec<i64>, nums: &[i32], left: usize, right:usize) -> i64 {
            let mid = left + (right - left) / 2;
            let num = nums[mid] as i64;
            num *((mid - left) as i64) - (sum[mid] - sum[left]) + (sum[right + 1] - sum[mid + 1]) - num * ((right - mid) as i64)
        }
        for i in 0..n {
            while distance(&sum, &nums, left, i) > k {
                left += 1;
            }
            ans = ans.max(i - left + 1);
        }
        ans as i32
    }

    pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut left = 0;
        let mut sum = 0i64;
        for right in 0..nums.len() {
            sum += nums[right] as i64 - nums[left + (right - left) / 2] as i64;
            while sum > k {
                sum += nums[left] as i64 - nums[(right + left + 1) / 2] as i64;
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,6,4]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,4,4,2,4]\n0\n
// @lcpr case=end

 */

