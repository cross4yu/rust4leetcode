/*
 * [2279] Maximum Bags With Full Capacity of Rocks
 */

// @lc code=start
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut diff: Vec<_> = capacity.into_iter().zip(rocks.into_iter()).map(|(a, b)| a - b ).collect();
        diff.sort_unstable();
        let mut rest = additional_rocks;
        diff.into_iter().fold(0, |ans, d| { if d <= rest { rest -= d; ans + 1 } else { ans } })
    }

    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut diff: Vec<_> = capacity.into_iter().zip(rocks.into_iter()).map(|(a, b)| a - b ).collect();
        diff.sort_unstable();
        let mut rest = additional_rocks;
        // fun 1
        let mut ans = 0;
        for i in 0..diff.len() {
            if rest >= diff[i] {
                rest -= diff[i];
                ans += 1;
            } else {
                return ans;
            }
        }
        ans as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [2,3,4,5]\n[1,2,4,4]\n2\n
// @lcpr case=end

// @lcpr case=start
// [10,2,2]\n[2,2,0]\n100\n
// @lcpr case=end

 */

