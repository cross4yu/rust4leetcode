/*
 * [3074] Apple Redistribution into Boxes
 */

// @lc code=start
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(a));
        let mut sum :i32 = apple.iter().sum();
        let mut ans = 0;
        while sum > 0 && ans < capacity.len() {
            sum -= capacity[ans];
            ans += 1;
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,2]\n[4,3,1,5,2]\n
// @lcpr case=end

// @lcpr case=start
// [5,5,5]\n[2,4,2,7]\n
// @lcpr case=end

 */

