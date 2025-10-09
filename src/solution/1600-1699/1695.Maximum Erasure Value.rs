/*
 * [1695] Maximum Erasure Value
 */

// @lc code=start
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; nums.len()];
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        let mut left = 0;
        let mut sum = 0;
        for &v in &nums {
            sum += v;
            *cnt.entry(v).or_insert(0) += 1;
            while cnt.get(&v).unwrap() > &1 {
                sum -= nums[left];
                cnt.entry(nums[left]).and_modify(|v| *v -= 1);
                left += 1;
            }
            res = res.max(sum);
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [4,2,4,5,6]\n
// @lcpr case=end

// @lcpr case=start
// [5,2,1,2,5,2,1,2,5]\n
// @lcpr case=end

 */

