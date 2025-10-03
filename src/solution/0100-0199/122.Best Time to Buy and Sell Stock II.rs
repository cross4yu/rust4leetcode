/*
 * [122] Best Time to Buy and Sell Stock II
 */

// @lc code=start
impl Solution {
    // Greedy Algorithm
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for (i, &v) in prices.iter().enumerate() {
            if i == 0 {
                continue;
            }
            profit += 0.max(v - prices[i - 1]);
        }
        profit
    }

    // rust
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        prices
            .iter()
            .zip(prices.iter().skip(1))
            .map(|(&pre, &cur)| 0.max(cur - pre))
            .sum()
    }

    // rust
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2) // [ [p0,p1], [p1,p2], ... ]
            .map(|w| 0.max(w[1] - w[0]))
            .sum()
    }

    // rust
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        prices.iter().skip(1).fold((prices[0], 0), |(prev, profit), &curr| {
            (curr, profit + 0.max(curr - prev))
        }).1 // 返回累加后的利润
    }

    // dp
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp0 = 0;
        let mut dp1 = i32::MIN;
        for p in &prices {
            let temp = dp0.max(dp1 + *p);
            dp1 = dp1.max(dp0 - *p);
            dp0 = temp;
        }
        dp0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [7,1,5,3,6,4]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n
// @lcpr case=end

// @lcpr case=start
// [7,6,4,3,1]\n
// @lcpr case=end

 */
