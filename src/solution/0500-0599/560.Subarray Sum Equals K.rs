/*
 * [560] Subarray Sum Equals K
 */

// @lc code=start
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let mut cnt = std::collections::HashMap::with_capacity(nums.len() + 1);
        for &num in &nums {
            *cnt.entry(sum).or_insert(0) += 1;
            sum += num;
            if let Some(&count) = cnt.get(&(sum - k)) {
                ans += count;
            }
        }
        ans
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let mut cnt = std::collections::HashMap::with_capacity(nums.len() + 1);
        cnt.insert(0, 1);
        for &num in &nums {
            sum += num;
            if let Some(&idx) = cnt.get(&(sum - k)) {
                ans += idx;
            }
            *cnt.entry(sum).or_insert(0) += 1;
        }
        ans
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] += sum[i] + nums[i];
        }
        let mut cnt = std::collections::HashMap::with_capacity(nums.len() + 1);
        for &s in &sum {
            if let Some(&idx) = cnt.get(&(s - k)) {
                ans += idx;
            }
            *cnt.entry(s).or_insert(0) += 1;
        }
        ans
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,1,1]\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n3\n
// @lcpr case=end

 */

