/*
 * [2271] Maximum White Tiles Covered by a Carpet
 */

// @lc code=start
impl Solution {
    pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        tiles.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 0;
        let mut left = 0;
        let mut sum = 0;
        for tile in tiles.iter() {
            sum += tile[1] + 1 - tile[0];
            let carpet_left = tile[1] + 1 - carpet_len;
            while tiles[left][1] < carpet_left {
                sum -= tiles[left][1] + 1 - tiles[left][0];
                left += 1;
            }
            let uncovered_sum = 0.max(carpet_left - tiles[left][0]);
            res = res.max(sum - uncovered_sum);
        }
        res as i32
    }

    // problem: Testcase [[1,1000000000]]  1000000000
    // memory allocation of 4000000004 bytes failed
    pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        tiles.sort_by(|a, b| a[0].cmp(&b[0]));
        let len = tiles[tiles.len() - 1][tiles[tiles.len() - 1].len() - 1];
        let mut nums = vec![0; len as usize + 1];
        for tile in tiles {
            for i in (tile[0] as usize) ..=(tile[tile.len() - 1] as usize) {
                nums[i as usize] = 1;
            }
        }
        // println!("{:?}", nums);
        let mut sum = 0;
        let mut left = 0;
        let mut res = 0;
        for (right, &v) in nums.iter().enumerate() {
            sum += v;
            if right - left + 1 > carpet_len as usize {
                sum -= nums[left];
                left += 1;
            }
            res = res.max(sum);
        }
        res as i32
    }
}
// @lc code=end



/*
// @lcpr case=start
// [[1,5],[10,11],[12,18],[20,25],[30,32]]\n10\n
// @lcpr case=end

// @lcpr case=start
// [[10,11],[1,1]]\n2\n
// @lcpr case=end

 */

