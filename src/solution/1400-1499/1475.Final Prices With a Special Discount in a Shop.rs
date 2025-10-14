/*
 * [1475] Final Prices With a Special Discount in a Shop
 */

// @lc code=start
impl Solution {
    // left to right deque
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = prices.clone();
        let mut queue = std::collections::VecDeque::new();
        for (i, &price) in prices.iter().enumerate() {
            while !queue.is_empty() && prices[i] <= prices[*queue.back().unwrap()] {
                if let Some(last) = queue.pop_back() {
                    result[last] = prices[last] - prices[i];
                }
            }
            queue.push_back(i);
        }
        result
    }

    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut queue = std::collections::VecDeque::new();
        for i in 0..prices.len() {
            while !queue.is_empty() && prices[i] <= prices[*queue.back().unwrap()] {
                prices[queue.pop_back().unwrap()] -= prices[i];
            }
            queue.push_back(i);
        }
        prices
    }

    // left to right vec
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; prices.len()];
        let mut stack = vec![];
        for (i, &price) in prices.iter().enumerate() {
            while !stack.is_empty() && price <= prices[stack[stack.len() - 1]] {
                let last = stack.pop().unwrap();
                result[last] = prices[last] - prices[i];
            }
            stack.push(i);
        }
        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            result[i] = prices[i];
        }
        result
    }

    // right to left
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = prices.clone();
        let mut stack = vec![];
        for (i, &price) in prices.iter().enumerate().rev() {
            while !stack.is_empty() && price < prices[stack[stack.len() - 1]] {
                stack.pop();
            }
            if let Some(&last) = stack.last() {
                result[i] = prices[i] - prices[last];
            }
            stack.push(i);
        }
        result
    }
}
// @lc code=end



/*
// @lcpr case=start
// [8,4,6,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n
// @lcpr case=end

// @lcpr case=start
// [10,1,1,6]\n
// @lcpr case=end

 */

