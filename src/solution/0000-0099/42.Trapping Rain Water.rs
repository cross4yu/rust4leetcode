/*
 * [42] Trapping Rain Water
 */

// @lc code=start
impl Solution {
    // dp
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n <= 2 {
            return 0;
        }
        let mut left_max = vec!(0; n);
        let mut right_max = vec!(0; n);
        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(height[i]);
        }
        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }
        let mut res = 0;
        for i in 0..n {
            res += left_max[i].min(right_max[i]) - height[i];
        }
        res
    }


}
// @lc code=end



/*
// @lcpr case=start
// [0,1,0,2,1,0,1,3,2,1,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [4,2,0,3,2,5]\n
// @lcpr case=end

 */

