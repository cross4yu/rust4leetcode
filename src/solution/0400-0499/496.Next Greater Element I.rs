/*
 * [496] Next Greater Element I
 */

// @lc code=start
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let index = nums1.iter().enumerate().map(|(i, &num)| (num, i)).collect::<std::collections::HashMap<_, _>>();
        let mut result = vec![-1; nums1.len()];
        let mut stack = vec![];
        for &num in &nums2 {
            while let Some(&top) = stack.last() {
                if num <= top {
                    break;
                }
                if let Some(&idx) = index.get(&top) {
                    result[idx] = num;
                }
                stack.pop();
            }
            stack.push(num);
        }
        result
    }
}
// @lc code=end



/*
// @lcpr case=start
// [4,1,2]\n[1,3,4,2]\n
// @lcpr case=end

// @lcpr case=start
// [2,4]\n[1,2,3,4]\n
// @lcpr case=end

 */

