/*
 * [1710] Maximum Units on a Truck
 */

// @lc code=start
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut ans = 0;
        let mut remind = truck_size;
        for unit in 0..box_types.len() {
            if remind < box_types[unit][0] {
                ans += box_types[unit][1] * remind;
                break;
            }
            ans += box_types[unit][1] * box_types[unit][0];
            remind -= box_types[unit][0];
        }
        ans as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[1,3],[2,2],[3,1]]\n4\n
// @lcpr case=end

// @lcpr case=start
// [[5,10],[2,5],[4,7],[3,9]]\n10\n
// @lcpr case=end

 */

