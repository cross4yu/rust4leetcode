/*
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            if area > res {
                res = area;
            }
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,8,6,2,5,4,8,3,7]\n
// @lcpr case=end

// @lcpr case=start
// [1,1]\n
// @lcpr case=end

 */

