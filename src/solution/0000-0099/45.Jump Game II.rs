/*
 * [45] Jump Game II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut end = 0;
        let mut max_pos = 0;
        let len = nums.len();
        for (i, &x) in nums.iter().enumerate().take(len - 1) {
            if i > max_pos {
                return steps;
            }
            max_pos = max_pos.max(i + x as usize);
            if i == end {
                end = max_pos;
                steps += 1;
            }
            if end >= len - 1 {
                break;
            }
        }
        steps
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,3,1,1,4]\n
// @lcpr case=end

// @lcpr case=start
// [2,3,0,1,4]\n
// @lcpr case=end

 */

