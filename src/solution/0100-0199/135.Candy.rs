/*
 * [135] Candy
 */

// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n <= 1 {
            return n as _;
        }
        let mut candies = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }
        candies.iter().sum()
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,0,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,2]\n
// @lcpr case=end

 */

