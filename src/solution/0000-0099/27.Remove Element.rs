/*
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut index = nums.len() as i32 - 1;
        let mut cur = index;
        while index >= 0 {
            let idx = index as usize;
            if nums[idx] == val {
                nums.swap(idx, cur as usize);
                cur -= 1;
            }
            index -= 1;
        }
        cur + 1
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,2,2,3]\n3\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,2,3,0,4,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [2,2,2,2]\n2\n
// @lcpr case=end

 */

