/*
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return nums[n/2] as i32;
        }
        nums.sort_unstable();
        return nums[n/2] as i32;
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut ans, mut cnt) = (0, 0);
        for i in nums {
            if cnt == 0 || i == ans {
                ans = i;
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        ans as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [3,2,3]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,1,1,1,2,2]\n
// @lcpr case=end

 */

