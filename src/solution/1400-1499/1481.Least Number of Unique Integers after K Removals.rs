/*
 * [1481] Least Number of Unique Integers after K Removals
 */

// @lc code=start
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut freq = arr.iter()
            .fold(std::collections::HashMap::new(), |mut m, &x| { *m.entry(x).or_insert(0) += 1; m })
            .into_values()
            .collect::<Vec<_>>();
        freq.sort_unstable();
        freq.iter()
            .scan(0, |sum, &f| { *sum += f; Some(*sum) })
            .filter(|&s| s > k)
            .count() as i32
    }

    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for &num in &arr {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut sort = cnt.into_values().collect::<Vec<_>>();
        sort.sort_unstable();
        let mut sum = 0;
        sort.iter_mut().map(|x| {sum += *x; sum}).filter(|x| *x > k).count() as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [5,5,4]\n1\n
// @lcpr case=end

// @lcpr case=start
// [4,3,1,1,3,3,2]\n3\n
// @lcpr case=end

 */

