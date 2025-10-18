/*
 * [2602] Minimum Operations to Make All Array Elements Equal
 */

// @lc code=start
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let len = nums.len();
        let mut sum = vec![0i64; len + 1];
        nums.sort_unstable();
        for i in 0..len {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let size = queries.len();
        let mut ans = vec![0i64; size];
        for i in 0..size {
            let q = queries[i];
            // let mid = nums.iter().filter(|&&x| x <= q).count(); // o(n)
            let mid = nums.partition_point(|&x| x <= q); // o(log(n))
            let left = q as i64 * mid as i64 - sum[mid];
            let right = sum[len] - sum[mid] - q as i64 * (len - mid) as i64;
            ans[i] = left + right;
        }
        ans
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,1,6,8]\n[1,5]\n
// @lcpr case=end

// @lcpr case=start
// [2,9,6,3]\n[10]\n
// @lcpr case=end

 */

