/*
 * [134] Gas Station
 */

// @lc code=start
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut balance = 0;
        let mut min_balance = i32::MAX;
        let mut idx = 0;
        for (i, &cost) in cost.iter().enumerate() {
            balance += gas[i] - cost;
            if balance < min_balance {
                min_balance = balance;
                idx = i;
            }
        }
        if balance >= 0  {(idx as i32 + 1) % gas.len() as i32} else {-1}
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,4,5]\n[3,4,5,1,2]\n
// @lcpr case=end

// @lcpr case=start
// [2,3,4]\n[3,4,3]\n
// @lcpr case=end

 */

