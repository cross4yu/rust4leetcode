/*
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    // first to last
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut cost = i32::MAX;
        for price in prices {
            cost = cost.min(price);
            profit = profit.max(price - cost);
        }
        profit
    }

    // last to first
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut right = 0;
        let mut len = prices.len() as i32 - 1;
        while len >= 0 {
            let cur = len as usize;
            right = right.max(prices[cur]);
            profit = profit.max(right - prices[cur]);
            len -= 1;
        }
        profit
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut right = 0;
        for &price in prices.iter().rev() {
            right = right.max(price);
            profit = profit.max(right - price);
        }
        profit
    }
}
// @lc code=end



/*
// @lcpr case=start
// [7,1,5,3,6,4]\n
// @lcpr case=end

// @lcpr case=start
// [7,6,4,3,1]\n
// @lcpr case=end

 */

