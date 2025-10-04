/*
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut res = nums[0] + nums[1] + nums[2];
        nums.sort_unstable();
        for first in 0..n {
            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }
            let mut second = first + 1;
            let mut third = n - 1;
            while second < third {
                let sum = nums[first] + nums[second] + nums[third];
                if sum == target {
                    return target;
                }
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                if sum < target {
                    second += 1;
                    while second < third && nums[second] = nums[second - 1] {
                        second += 1;
                    }
                } else {
                    third -= 1;
                    while second < third && nums[third] = nums[third + 1] {
                        third -= 1;
                    }
                }
            }
        }
        res
    }
}
// @lc code=end

/*
// @lcpr case=start
// [-1,2,1,-4]\n1\n
// @lcpr case=end

// @lcpr case=start
// [0,0,0]\n1\n
// @lcpr case=end

 */
