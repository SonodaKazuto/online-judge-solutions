// LeetCode 374. Guess Number Higher or Lower
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        if guess(n) == 0 { return n; }

        let mut left: i32 = 1;
        let mut right: i32 = n;
        loop {
            let mid: i32 = left + (right - left) / 2;
            match guess(mid) {
                1 => left = mid,
                -1 => right = mid,
                _ => return mid
            }
        }

        return 0;
    }
}
