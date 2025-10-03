/*
 * [88] Merge Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.resize((m + n) as usize, 0);
        let mut cur = m + n - 1;
        let mut p = m - 1;
        let mut q = n - 1;
        while q >= 0 {
            if p < 0 || nums1[p as usize] < nums2[q as usize] {
                nums1[cur as usize] = nums2[q as usize];
                q -= 1;
                cur -= 1;
            } else {
                nums1[cur as usize] = nums1[p as usize];
                p -= 1;
                cur -= 1;
            }
        }
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,0,0,0]\n3\n[2,5,6]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1]\n1\n[]\n0\n
// @lcpr case=end

// @lcpr case=start
// [0]\n0\n[1]\n1\n
// @lcpr case=end

 */

