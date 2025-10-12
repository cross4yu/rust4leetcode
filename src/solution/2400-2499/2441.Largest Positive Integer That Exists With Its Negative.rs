/*
 * [2441] Largest Positive Integer That Exists With Its Negative
 */

// @lc code=start
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&(-num)) {
                res = res.max(num.abs());
            }
            set.insert(num);
        }
        return res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [-1,2,-3,3]\n
// @lcpr case=end

// @lcpr case=start
// [-1,10,6,7,-7,1]\n
// @lcpr case=end

// @lcpr case=start
// [-10,8,6,7,-2,-3]\n
// @lcpr case=end

 */

