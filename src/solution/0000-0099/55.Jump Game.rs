/*
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    // Greedy Algorithm
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let target = nums.len() - 1;
        let mut max_pos = 0;
        for (i, &v) in nums.iter().enumerate() {
            if i > max_pos {
                return false;
            }
            max_pos = max_pos.max(v as usize + i);
            if max_pos >= target {
                return true;
            }
        }
        false
    }


}
// @lc code=end



/*
// @lcpr case=start
// [2,3,1,1,4]\n
// @lcpr case=end

// @lcpr case=start
// [3,2,1,0,4]\n
// @lcpr case=end

 */

