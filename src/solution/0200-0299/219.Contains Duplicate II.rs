/*
 * [219] Contains Duplicate II
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
        for (i, &num) in nums.iter().enumerate() {
            if !set.insert(num) {
                return true;
            }
            if i >= k {
                set.remove(&nums[i - k]);
            }
        }
        false
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            match cnt.entry(num) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if i - *e.get() <= k as usize {
                        return true;
                    }
                }
                std::collections::hash_map::Entry::Vacant(e) => {}
            }
            cnt.insert(num, i);
        }
        false
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = cnt.get(&(num)) {
                if i - j <= k as usize {
                    return true;
                }
            }
            cnt.insert(num, i);
        }
        false
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,1]\n3\n
// @lcpr case=end

// @lcpr case=start
// [1,0,1,1]\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,1,2,3]\n2\n
// @lcpr case=end

 */

