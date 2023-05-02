// LeetCode 744. Find Smallest Letter Greater Than Target
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        return letters.clone()
                      .into_iter()
                      .filter(|&c| c > target)
                      .min()
                      .unwrap_or(*letters.first().unwrap());
    }
}
