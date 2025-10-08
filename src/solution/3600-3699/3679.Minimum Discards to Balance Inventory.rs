/*
 * [3679]  Minimum Discards to Balance Inventory
 */

// @lc code=start
impl Solution {
    pub fn min_arrivals_to_discard(mut arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        let w = w as usize;
        for i in 0..arrivals.len() {
            let cur = arrivals[i];
            *cnt.entry(cur).or_insert(0) += 1;
            if *cnt.get(&cur).unwrap() > m {
                res += 1;
                cnt.entry(cur).and_modify(|v| *v -= 1);
                arrivals[i] = 0;
            }
            if i + 1 < w {
                continue;
            }
            let out = arrivals[i + 1 - w];
            if out != 0 {
                match cnt.entry(out) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        *e.get_mut() -= 1;
                        if *e.get() <= 0 {
                            cnt.remove(&out);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {}
                }
            }
        }
        res
    }

    pub fn min_arrivals_to_discard(mut arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut res = 0;
        let w = w as usize;
        for i in 0..arrivals.len() {
            let cur = arrivals[i];
            if *cnt.get(&cur).unwrap_or_else(|| &0) >= m {
                res += 1;
                arrivals[i] = 0;
            } else {
                *cnt.entry(cur).or_insert(0) += 1;
            }
            if i + 1 < w {
                continue;
            }
            let out = arrivals[i + 1 - w];
            if out != 0 {
                match cnt.entry(out) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        *e.get_mut() -= 1;
                        if *e.get() <= 0 {
                            cnt.remove(&out);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {}
                }
            }
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,1,3,1]\n4\n2\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,3,3,4]\n3\n2\n
// @lcpr case=end

 */

