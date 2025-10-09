/*
 * [2958] Length of Longest Subarray With at Most K Frequency
 */

// @lc code=start
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        let mut res = 0;
        let mut left = 0;
        for (right, & num) in nums.iter().enumerate() {
            *cnt.entry(num).or_insert(0) += 1;
            while cnt.get(&num).unwrap() > &k {
                cnt.entry(nums[left]).and_modify(|v| *v -= 1);
                left += 1;
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,1,2,3,1,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,2,1,2,1,2,1,2]\n1\n
// @lcpr case=end

// @lcpr case=start
// [5,5,5,5,5,5,5]\n4\n
// @lcpr case=end

 */

