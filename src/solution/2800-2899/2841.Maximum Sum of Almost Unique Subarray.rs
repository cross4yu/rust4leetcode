/*
 * [2841] Maximum Sum of Almost Unique Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let mut cnt = std::collections::HashMap::new();
        let m = m as usize;
        let k = k as usize;
        let mut sum: i64 = 0;
        let mut res: i64 = 0;
        for (i, &v) in nums.iter().enumerate() {
            sum += v as i64;
            *cnt.entry(v).or_insert(0) += 1;
            if i + 1 < k {
                continue;
            }
            if cnt.len() >= m {
                res = res.max(sum);
            }
            let out = nums[i + 1 -k];
            sum -= out as i64;
            match cnt.entry(out) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    *e.get_mut() -= 1;
                    if *e.get() == 0 {
                        cnt.remove(&out);
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
// [2,6,7,3,1,7]\n3\n4\n
// @lcpr case=end

// @lcpr case=start
// [5,9,9,2,4,5,4]\n1\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,2,1,2,1,2,1]\n3\n3\n
// @lcpr case=end

 */

