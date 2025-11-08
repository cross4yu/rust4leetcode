/*
 * [2037] Minimum Number of Moves to Seat Everyone
 */

// @lc code=start
impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        let mut ans = 0;
        for i in 0..students.len() {
            ans += (students[i] - seats[i]).abs();
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,1,5]\n[2,7,4]\n
// @lcpr case=end

// @lcpr case=start
// [4,1,5,9]\n[1,3,2,6]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,6,6]\n[1,3,2,6]\n
// @lcpr case=end

 */

