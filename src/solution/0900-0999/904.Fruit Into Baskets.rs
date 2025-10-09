/*
 * [904] Fruit Into Baskets
 */

// @lc code=start
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut cnt = std::collections::HashMap::new();
        for (right, &v) in fruits.iter().enumerate() {
            *cnt.entry(v).or_insert(0) += 1;
            while cnt.len() > 2 {
                let out = fruits[left];
                match cnt.entry(out) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        *e.get_mut() -= 1;
                        if *e.get() == 0 {
                            e.remove();
                        }
                    },
                    std::collections::hash_map::Entry::Vacant(e) => {}
                }
                left += 1;
            }
            res = res.max(right + 1 - left);
        }
        return res as _
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,2]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,2,2]\n
// @lcpr case=end

 */

