/*
 * [1423] Maximum Points You Can Obtain from Cards
 */

// @lc code=start
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len = card_points.len();
        let windows = len - k as usize;
        if windows == 0 {
            return card_points.iter().sum();
        }
        let mut sum = 0;
        let mut res = i32::MAX;
        for (i, &v) in card_points.iter().enumerate() {
            sum += v;
            if i + 1 < windows {
                continue;
            }
            res = res.min(sum);
            sum -= card_points[i + 1 - windows];
        }
        card_points.iter().sum::<i32>() - res
    }

    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len = card_points.len();
        let windows = len - k as usize;
        let mut sum = card_points.iter().take(windows).sum::<i32>();
        let mut min = sum;
        for i in windows..len {
            sum += card_points[i] - card_points[i - windows];
            min = min.min(sum);
        }
        card_points.iter().sum::<i32>() - min
    }

    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut sum = card_points.iter().take(k).sum::<i32>();
        let mut res = sum;
        for i in 1..=k {
            sum += card_points[card_points.len() - i] - card_points[k - i];
            res = res.max(sum);
        }
        res
    }
}
// @lc code=end



/*
// @lcpr case=start
// [1,2,3,4,5,6,1]\n3\n
// @lcpr case=end

// @lcpr case=start
// [2,2,2]\n2\n
// @lcpr case=end

// @lcpr case=start
// [9,7,7,9,7,7,9]\n7\n
// @lcpr case=end

 */

