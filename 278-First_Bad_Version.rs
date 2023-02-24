// LeetCode 278. First Bad Version
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		if n == 1 { return 1 }

        let (mut low, mut high) = (1, n);

        while low != high {
            let mid = (low + high) / 2;
            match self.isBadVersion(mid) {
                true => high = mid,
                false => low = mid + 1
            }
        }

        return low;
    }
}
