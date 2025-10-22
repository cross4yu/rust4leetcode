/*
 * [1524] Number of Sub-arrays With Odd Sum
 */

// @lc code=start
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans: i64 = 0;
        let mut sum = 0;
        let (mut odd, mut even) = (0, 1);
        for &num in &arr {
            sum += num as i64;
            let is_even = sum & 1 == 0;
            ans = (ans + (if is_even {odd} else {even})) % MOD;
            if is_even {even += 1;} else {odd += 1;}
        }
        (ans % MOD) as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,5]\n
// @lcpr case=end

// @lcpr case=start
// [2,4,6]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5,6,7]\n
// @lcpr case=end

 */

