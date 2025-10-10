/*
 * [2831] Find the Longest Equal Subarray
 */

// @lc code=start
impl Solution {
    // hashmap
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k = k as usize;
        let mut pos = std::collections::HashMap::<i32, Vec<usize>>::new();
        for (i, &num) in nums.iter().enumerate() {
            pos.entry(num).or_default().push(i);
        }

        for pos in pos.values() {
            let mut left = 0;
            for right in 0..pos.len() {
                while pos[right] - pos[left] - (right - left) > k {
                    left += 1;
                }
                res = res.max((right + 1 - left) as i32);
            }
        }
        res as i32
    }

    // left right pointer
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        for (right , &v) in nums.iter().enumerate() {
            *cnt.entry(v).or_insert(0) += 1;
            while (right + 1 - left) as i32 > k + cnt.get(&nums[left]).unwrap() {
                cnt.entry(nums[left]).and_modify(|v| *v -= 1);
                left += 1;
            }
            res = res.max(cnt[&nums[right]]);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,3,2,3,1,3]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,1,2,2,1,1]\n2\n
// @lcpr case=end

 */

