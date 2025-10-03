// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum3(nums:Vec<i32>, target:i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&v) = map.get(&(target - num)) {
                return vec![i as i32, v]
            }
            map.insert(num, i);
        }
        return Vec::new()
    }
}
// @lc code=end


/*
// @lcpr case=start
// [2,7,11,15]\n9\n
// @lcpr case=end

// @lcpr case=start
// [3,2,4]\n6\n
// @lcpr case=end

// @lcpr case=start
// [3,3]\n6\n
// @lcpr case=end

 */