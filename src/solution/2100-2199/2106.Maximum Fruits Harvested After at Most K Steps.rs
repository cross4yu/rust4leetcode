/*
 * [2106] Maximum Fruits Harvested After at Most K Steps
 */

// @lc code=start
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut sum = 0;
        for right in 0..fruits.len() {
            sum += fruits[right][1];
            while left <= right && fruits[right][0] - fruits[left][0] + (start_pos - fruits[left][0]).abs().min((start_pos - fruits[right][0]).abs()) > k {
                sum -= fruits[left][1];
                left += 1;
            }
            res = res.max(sum);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[2,8],[6,3],[8,6]]\n5\n4\n
// @lcpr case=end

// @lcpr case=start
// [[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]]\n5\n4\n
// @lcpr case=end

// @lcpr case=start
// [[0,3],[6,4],[8,5]]\n3\n2\n
// @lcpr case=end

 */

