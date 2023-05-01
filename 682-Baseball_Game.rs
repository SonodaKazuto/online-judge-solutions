// LeetCode 682. Baseball Game
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        if operations.is_empty() { return 0; }

        let mut score_list: Vec<i32> = Vec::new();
        for op in operations.into_iter() {
            if op.as_str() == "+" {
                if score_list.len() < 2 { return -1; } 
                score_list.push(score_list[score_list.len() - 1] + score_list[score_list.len() - 2]);
            } else if op.as_str() == "D" {
                if score_list.len() < 1 { return -1; }
                score_list.push(score_list.last().unwrap() * 2);
            } else if op.as_str() == "C" { score_list.pop(); }
            else { score_list.push(op.to_string().parse::<i32>().unwrap()); }
        }

        return score_list.iter().sum::<i32>();
    }
}
