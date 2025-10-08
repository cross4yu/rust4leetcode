/*
 * [2461] Maximum Sum of Distinct Subarrays With Length K
 */

// @lc code=start
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut res: i64 = 0;
        let mut sum: i64 = 0;
        let mut cnt = std::collections::HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            sum += v as i64;
            *cnt.entry(v).or_insert(0) += 1;
            if i + 1 < k {
                continue;
            }
            if cnt.len() == k {
                res = res.max(sum);
            }
            let out = nums[i + 1 -k];
            sum -= out as i64;
            match cnt.entry(out) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    *e.get_mut() -= 1;
                    if *e.get() == 0 {
                        e.remove();
                    }
                }
                std::collections::hash_map::Entry::Vacant(e) => {}
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,5,4,2,9,9,9]\n3\n
// @lcpr case=end

// @lcpr case=start
// [4,4,4]\n3\n
// @lcpr case=end

 */

