/*
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();
        for first in (0..nums.len()) {
            if first > 0 && nums[first - 1] == nums[first] {
                continue;
            }
            let target = 0 - nums[first];
            let mut second = first + 1;
            let mut third = nums.len() - 1;
            while second < third {
                let sum = nums[second] + nums[third];
                if sum == target {
                    result.push(vec![nums[first], nums[second], nums[third]]);
                    second += 1;
                    third -= 1;
                    while second < third && nums[second] == nums[second - 1] {
                        second += 1;
                    }
                    while second < third && nums[third] == nums[third + 1] {
                        third -= 1;
                    }
                } else if sum > target {
                    third -= 1;
                } else {
                    second += 1;
                }
            }
        }
        result
    }
}
// @lc code=end



/*
// @lcpr case=start
// [-1,0,1,2,-1,-4]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [0,0,0]\n
// @lcpr case=end

 */

