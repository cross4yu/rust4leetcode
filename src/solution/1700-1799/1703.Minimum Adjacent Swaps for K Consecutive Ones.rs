/*
 * [1703] Minimum Adjacent Swaps for K Consecutive Ones
 */

// @lc code=start
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut p = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                p.push((i - p.len()) as i32);
            }
        }
        let m = p.len();
        let mut sum = vec![0i32; m + 1];
        for i in 0..m {
            sum[i + 1] = sum[i] + p[i];
        }
        let mut ans = i32::MAX;
        for i in 0..=(m - k) {
            ans = ans.min(sum[i] + sum[i + k] - sum[i + k / 2] * 2 - p[i + k / 2] * ((k % 2) as i32));
        }
        ans
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,0,0,1,0,1]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,0,0,0,0,0,1,1]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,1,0,1]\n2\n
// @lcpr case=end

 */

