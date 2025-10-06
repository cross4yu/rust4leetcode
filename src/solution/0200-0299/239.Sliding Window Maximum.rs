/*
 * [239] Sliding Window Maximum
 */

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        let k = k as usize;
        if len < k  {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(len - k + 1);
        let mut deque = VecDeque::new();
        for (i, &num) in nums.iter().enumerate() {
            while !deque.is_empty() && num >= nums[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(i);
            // note: i k is usize, watch out for overflow!!!
            if deque[0] + k <= i {
                deque.pop_front();
            }
            if i + 1 >= k {
                res.push(nums[deque[0]]);
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,-1,-3,5,3,6,7]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1]\n1\n
// @lcpr case=end

 */

