/*
 * [380] Insert Delete GetRandom O(1)
 */

// @lc code=start

use std::collections::HashMap;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
struct RandomizedSet {
    values: Vec<i32>,
    idx: HashMap<i32, usize>,
    rng: ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            values: Vec::with_capacity(32),
            idx: HashMap::with_capacity(32),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.idx.contains_key(&val) {
            true => false,
            false => {
                self.values.push(val);
                self.idx.insert(val, self.values.len() - 1);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.idx.remove(&val) {
            Some(idx) => {
                let last = self.values[self.values.len() - 1];
                if last != val {
                    self.values[idx] = last;
                    *self.idx.entry(last).or_insert(0) = idx;
                }
                self.values.pop();
                true
            },
            None => false,
        }
    }

    fn get_random(&mut self) -> i32 {
        self.values[self.rng.gen_range(0..self.values.len())]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end



