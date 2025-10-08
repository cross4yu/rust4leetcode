/*
 * [1052] Grumpy Bookstore Owner
 */

// @lc code=start
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let k = minutes as usize;
        let mut satisfied = 0;
        let mut angry = 0;
        let mut max_satisfied = 0;
        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                satisfied += customers[i];
            } else {
                angry += customers[i];
            }
            if i + 1 < k {
                continue;
            }
            max_satisfied = max_satisfied.max(angry);
            if grumpy[i + 1 - k] == 1 {
                angry -= customers[i + 1 - k];
            }
        }
        satisfied + max_satisfied
    }

    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let k = minutes as usize;
        let mut satisfied = vec![0; 2];
        let mut max_satisfied = 0;
        for i in 0..customers.len() {
            satisfied[grumpy[i] as usize] += customers[i];
            if i + 1 < k {
                continue;
            }
            max_satisfied = max_satisfied.max(satisfied[1]);
            if grumpy[i + 1 - k] == 1 {
                satisfied[1] -= customers[i + 1 - k];
            }
        }
        satisfied[0] + max_satisfied
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,0,1,2,1,1,7,5]\n[0,1,0,1,0,1,0,1]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1]\n[0]\n1\n
// @lcpr case=end

 */

