/*
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {

    // left to right
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (i, &temp) in temperatures.iter().enumerate() {
            while !stack.is_empty() && temperatures[stack.last().unwrap_or(0)] < temp {
                let last = stack.pop().unwrap();
                res[last as usize] = (i - last) as i32;
            }
            stack.push(i);
        }
        res
    }

    // right to left
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (i, &temp) in temperatures.iter().enumerate().rev() {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] <= temp {
                stack.pop();
            }
            if let Some(last) = stack.last() {
                res[i] = (last - i) as i32;
            }
            stack.push(i);
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [73,74,75,71,69,72,76,73]\n
// @lcpr case=end

// @lcpr case=start
// [30,40,50,60]\n
// @lcpr case=end

// @lcpr case=start
// [30,60,90]\n
// @lcpr case=end

 */

