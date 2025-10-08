/*
 * [1343] Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
 */

// @lc code=start
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let target = k * threshold;
        let k = k as usize;
        let mut sum = 0;
        let mut res = 0;
        for (i, &v) in arr.iter().enumerate() {
            sum += v;
            if i + 1 < k {
                continue;
            }
            if sum >= target {
                res += 1;
            }
            sum -= arr[i + 1 - k];
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,2,2,2,5,5,5,8]\n3\n4\n
// @lcpr case=end

// @lcpr case=start
// [11,13,17,23,29,31,7,5,2,3]\n3\n5\n
// @lcpr case=end

 */

