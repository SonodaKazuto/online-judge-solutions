use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut relations: Vec<i32> = nums.windows(2)
                                          .filter_map(|w| match w[1].cmp(&w[0]) {
                                              Ordering::Greater => Some(1),
                                              Ordering::Less => Some(-1),
                                              _ => None
                                          })
                                          .collect::<_>();

        relations.sort_unstable();
        relations.dedup();

        return (relations.len() == 1) || relations.is_empty();
    }
}
