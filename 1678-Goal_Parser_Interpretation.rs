// LeetCode 1678. Goal Parser Interpretation
impl Solution {
    pub fn interpret(command: String) -> String {
        return command.clone().replace("()", "o").replace("(al)", "al");
    }
}
